use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

struct Exercise {
    id: String,
    path: PathBuf,
    has_tests: bool,
}

struct CheckSummary {
    passed: usize,
    failed: usize,
    manual: usize,
}

enum Action {
    Help,
    List,
    Check(Option<String>),
}

enum CheckerError {
    Io(String),
    Usage(String),
}

impl fmt::Display for CheckerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(message) | Self::Usage(message) => formatter.write_str(message),
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    match run(&arguments) {
        Ok(Some(summary)) if summary.failed > 0 => process::exit(1),
        Ok(_) => {}
        Err(error) => {
            eprintln!("Erreur : {error}");
            process::exit(2);
        }
    }
}

fn run(arguments: &[String]) -> Result<Option<CheckSummary>, CheckerError> {
    let action = parse_action(arguments)?;
    let exercises = collect_exercises()?;

    match action {
        Action::Help => print_help(),
        Action::List => print_exercises(&exercises),
        Action::Check(selector) => return check_exercises(&exercises, selector.as_deref()).map(Some),
    }

    Ok(None)
}

fn parse_action(arguments: &[String]) -> Result<Action, CheckerError> {
    match arguments {
        [] => Ok(Action::Help),
        [argument] if matches!(argument.as_str(), "help" | "--help" | "-h") => Ok(Action::Help),
        [command] if command == "list" => Ok(Action::List),
        [command] if command == "check" => Ok(Action::Check(None)),
        [command, selector] if command == "check" => Ok(Action::Check(Some(selector.clone()))),
        _ => Err(CheckerError::Usage(
            "Utilisation : cargo run --bin exercices -- [list|check [cible]]".to_string(),
        )),
    }
}

fn print_help() {
    println!("Verifie les exercices Rust sans compiler toute la piscine en un seul programme.");
    println!();
    println!("Utilisation :");
    println!("  cargo run --bin exercices -- list");
    println!("  cargo run --bin exercices -- check");
    println!("  cargo run --bin exercices -- check day1");
    println!("  cargo run --bin exercices -- check day1/variables1");
    println!();
    println!("Legende :");
    println!("  {GREEN}[OK]{RESET} tests passes");
    println!("  {RED}[KO]{RESET} compilation ou tests en echec");
    println!("  {YELLOW}[??]{RESET} aucun test embarque, verification manuelle necessaire");
}

fn print_exercises(exercises: &[Exercise]) {
    println!("Exercices disponibles :");
    for exercise in exercises {
        let test_status = if exercise.has_tests {
            "tests embarques"
        } else {
            "verification manuelle"
        };
        println!("  {:<38} {test_status}", exercise.id);
    }
}

fn check_exercises(
    exercises: &[Exercise],
    selector: Option<&str>,
) -> Result<CheckSummary, CheckerError> {
    ensure_rustc_is_available()?;

    let selected: Vec<&Exercise> = exercises
        .iter()
        .filter(|exercise| selector.map_or(true, |value| matches_selector(exercise, value)))
        .collect();

    if selected.is_empty() {
        return Err(CheckerError::Usage(format!(
            "Aucun exercice ne correspond a `{}`. Utilisez `cargo run --bin exercices -- list`.",
            selector.unwrap_or_default()
        )));
    }

    let output_directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/exercise-checker");
    fs::create_dir_all(&output_directory).map_err(|error| {
        CheckerError::Io(format!(
            "Impossible de creer `{}` : {error}",
            output_directory.display()
        ))
    })?;

    let mut summary = CheckSummary {
        passed: 0,
        failed: 0,
        manual: 0,
    };

    for exercise in selected {
        let binary = output_path(&output_directory, exercise);
        let result = if exercise.has_tests {
            check_tested_exercise(exercise, &binary)
        } else {
            check_untested_exercise(exercise, &binary)
        };

        match result {
            Ok(CheckResult::Passed) => {
                summary.passed += 1;
                print_status(GREEN, "OK", exercise, "tests passes");
            }
            Ok(CheckResult::Manual) => {
                summary.manual += 1;
                print_status(
                    YELLOW,
                    "??",
                    exercise,
                    "compile, mais aucun test embarque",
                );
            }
            Err(error) => {
                summary.failed += 1;
                print_status(RED, "KO", exercise, &error);
            }
        }
    }

    println!();
    println!(
        "Bilan : {GREEN}{} valides{RESET}, {RED}{} en echec{RESET}, {YELLOW}{} a verifier manuellement{RESET}",
        summary.passed, summary.failed, summary.manual
    );

    Ok(summary)
}

enum CheckResult {
    Passed,
    Manual,
}

fn check_tested_exercise(exercise: &Exercise, binary: &Path) -> Result<CheckResult, String> {
    let compilation = Command::new("rustc")
        .arg("--edition")
        .arg("2021")
        .arg("--test")
        .arg(&exercise.path)
        .arg("-o")
        .arg(binary)
        .status()
        .map_err(|error| format!("impossible de lancer rustc : {error}"))?;

    if !compilation.success() {
        return Err("la compilation des tests a echoue".to_string());
    }

    let tests = Command::new(binary)
        .arg("--color")
        .arg("never")
        .status()
        .map_err(|error| format!("impossible d'executer les tests : {error}"))?;

    if tests.success() {
        Ok(CheckResult::Passed)
    } else {
        Err("au moins un test echoue".to_string())
    }
}

fn check_untested_exercise(exercise: &Exercise, binary: &Path) -> Result<CheckResult, String> {
    let compilation = Command::new("rustc")
        .arg("--edition")
        .arg("2021")
        .arg(&exercise.path)
        .arg("-o")
        .arg(binary)
        .status()
        .map_err(|error| format!("impossible de lancer rustc : {error}"))?;

    if compilation.success() {
        Ok(CheckResult::Manual)
    } else {
        Err("la compilation a echoue".to_string())
    }
}

fn ensure_rustc_is_available() -> Result<(), CheckerError> {
    match Command::new("rustc")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(CheckerError::Io(format!(
            "rustc a echoue pendant sa verification ({status})"
        ))),
        Err(error) => Err(CheckerError::Io(format!(
            "rustc est requis pour verifier les exercices : {error}"
        ))),
    }
}

fn collect_exercises() -> Result<Vec<Exercise>, CheckerError> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/exercices");
    let mut exercises = Vec::new();
    collect_from_directory(&root, &root, &mut exercises)?;
    exercises.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(exercises)
}

fn collect_from_directory(
    directory: &Path,
    root: &Path,
    exercises: &mut Vec<Exercise>,
) -> Result<(), CheckerError> {
    let entries = fs::read_dir(directory).map_err(|error| {
        CheckerError::Io(format!(
            "Impossible de lire `{}` : {error}",
            directory.display()
        ))
    })?;

    for entry in entries {
        let path = entry
            .map_err(|error| CheckerError::Io(format!("Impossible de lire une entree : {error}")))?
            .path();

        if path.is_dir() {
            collect_from_directory(&path, root, exercises)?;
            continue;
        }

        if path.extension().and_then(|extension| extension.to_str()) != Some("rs")
            || matches!(path.file_name().and_then(|name| name.to_str()), Some("main.rs" | "mod.rs"))
        {
            continue;
        }

        let relative_path = path.strip_prefix(root).expect("exercise path is below its root");
        let id = relative_path
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&path).map_err(|error| {
            CheckerError::Io(format!("Impossible de lire `{}` : {error}", path.display()))
        })?;

        exercises.push(Exercise {
            id,
            path,
            has_tests: source.contains("#[cfg(test)]"),
        });
    }

    Ok(())
}

fn matches_selector(exercise: &Exercise, selector: &str) -> bool {
    let selector = selector
        .trim_matches('/')
        .trim_end_matches(".rs")
        .replace('\\', "/");
    let short_id = exercise
        .id
        .split_once('/')
        .map(|(day, _)| format!(
            "{day}/{}",
            exercise
                .path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .expect("exercise source has a UTF-8 filename")
        ))
        .expect("exercise id contains its day");

    exercise.id == selector
        || short_id == selector
        || exercise.id.ends_with(&format!("/{selector}"))
        || exercise.id.starts_with(&format!("{selector}/"))
}

fn output_path(directory: &Path, exercise: &Exercise) -> PathBuf {
    directory.join(format!(
        "{}{}",
        exercise.id.replace('/', "_"),
        env::consts::EXE_SUFFIX
    ))
}

fn print_status(color: &str, label: &str, exercise: &Exercise, detail: &str) {
    println!("{color}[{label}] {}{RESET} - {detail}", exercise.id);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exercise(id: &str) -> Exercise {
        Exercise {
            id: id.to_string(),
            path: PathBuf::from(format!("src/exercices/{id}.rs")),
            has_tests: true,
        }
    }

    #[test]
    fn accepts_the_supported_commands() {
        assert!(matches!(parse_action(&[]), Ok(Action::Help)));
        assert!(matches!(parse_action(&["list".to_string()]), Ok(Action::List)));
        assert!(matches!(
            parse_action(&["check".to_string(), "day1".to_string()]),
            Ok(Action::Check(Some(selector))) if selector == "day1"
        ));
    }

    #[test]
    fn rejects_invalid_commands() {
        assert!(matches!(
            parse_action(&["unknown".to_string()]),
            Err(CheckerError::Usage(_))
        ));
    }

    #[test]
    fn matches_day_exact_and_short_selectors() {
        let exercise = exercise("day1/_01_variables/variables1");
        assert!(matches_selector(&exercise, "day1"));
        assert!(matches_selector(&exercise, "day1/_01_variables/variables1"));
        assert!(matches_selector(&exercise, "day1/variables1"));
        assert!(matches_selector(&exercise, "variables1"));
        assert!(!matches_selector(&exercise, "day2"));
    }
}
