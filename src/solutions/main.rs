use std::env;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, ExitStatus};

struct Solution {
    id: &'static str,
    source: &'static str,
}

const SOLUTIONS: &[Solution] = &[
    Solution { id: "day1/variables1", source: "day1/_01_variables/variables1.rs" },
    Solution { id: "day1/variables2", source: "day1/_01_variables/variables2.rs" },
    Solution { id: "day1/variables3", source: "day1/_01_variables/variables3.rs" },
    Solution { id: "day1/variables4", source: "day1/_01_variables/variables4.rs" },
    Solution { id: "day1/condition1", source: "day1/_02_conditions/condition1.rs" },
    Solution { id: "day1/condition2", source: "day1/_02_conditions/condition2.rs" },
    Solution { id: "day1/condition3", source: "day1/_02_conditions/condition3.rs" },
    Solution { id: "day1/function1", source: "day1/_03_functions/function1.rs" },
    Solution { id: "day1/function2", source: "day1/_03_functions/function2.rs" },
    Solution { id: "day1/function3", source: "day1/_03_functions/function3.rs" },
    Solution { id: "day1/boucle1", source: "day1/_04_boucles/boucle1.rs" },
    Solution { id: "day1/boucle2", source: "day1/_04_boucles/boucle2.rs" },
    Solution { id: "day1/boucle3", source: "day1/_04_boucles/boucle3.rs" },
    Solution { id: "day1/vector1", source: "day1/_05_vectors/vector1.rs" },
    Solution { id: "day1/vector2", source: "day1/_05_vectors/vector2.rs" },
    Solution { id: "day2/loop1", source: "day2/_06_iteration/loop1.rs" },
    Solution { id: "day2/loop2", source: "day2/_06_iteration/loop2.rs" },
    Solution { id: "day2/loop3", source: "day2/_06_iteration/loop3.rs" },
    Solution { id: "day2/loop4", source: "day2/_06_iteration/loop4.rs" },
    Solution { id: "day2/loop5", source: "day2/_06_iteration/loop5.rs" },
    Solution { id: "day2/pattern_matching1", source: "day2/_07_pattern_matching/pattern_matching1.rs" },
    Solution { id: "day2/pattern_matching2", source: "day2/_07_pattern_matching/pattern_matching2.rs" },
    Solution { id: "day2/pattern_matching3", source: "day2/_07_pattern_matching/pattern_matching3.rs" },
    Solution { id: "day2/string1", source: "day2/_08_strings/string1.rs" },
    Solution { id: "day2/string2", source: "day2/_08_strings/string2.rs" },
    Solution { id: "day2/string3", source: "day2/_08_strings/string3.rs" },
    Solution { id: "day2/hashmap1", source: "day2/_09_hashmaps/hashmap1.rs" },
    Solution { id: "day2/hashmap2", source: "day2/_09_hashmaps/hashmap2.rs" },
    Solution { id: "day2/struct1", source: "day2/_10_structs/struct1.rs" },
    Solution { id: "day2/struct2", source: "day2/_10_structs/struct2.rs" },
    Solution { id: "day3/trait1", source: "day3/_11_traits/trait1.rs" },
    Solution { id: "day3/trait2", source: "day3/_11_traits/trait2.rs" },
    Solution { id: "day3/ownership1", source: "day3/_12_ownership/ownership1.rs" },
    Solution { id: "day3/ownership2", source: "day3/_12_ownership/ownership2.rs" },
    Solution { id: "day3/ownership3", source: "day3/_12_ownership/ownership3.rs" },
    Solution { id: "day3/life_cycle1", source: "day3/_13_life_cycle/life_cycle1.rs" },
    Solution { id: "day3/life_cycle2", source: "day3/_13_life_cycle/life_cycle2.rs" },
    Solution { id: "day3/option1", source: "day3/_14_error_handling/option1.rs" },
    Solution { id: "day3/option2", source: "day3/_14_error_handling/option2.rs" },
    Solution { id: "day3/option3", source: "day3/_14_error_handling/option3.rs" },
    Solution { id: "day3/result1", source: "day3/_14_error_handling/result1.rs" },
    Solution { id: "day3/result2", source: "day3/_14_error_handling/result2.rs" },
    Solution { id: "day3/result3", source: "day3/_14_error_handling/result3.rs" },
    Solution { id: "day3/user_option", source: "day3/_14_error_handling/user_option.rs" },
    Solution { id: "day4/generic_type1", source: "day4/_15_generics_types/generic_type1.rs" },
    Solution { id: "day4/generic_type2", source: "day4/_15_generics_types/generic_type2.rs" },
    Solution { id: "day4/event_manager", source: "day4/_16_fn_pointers/event_manager.rs" },
    Solution { id: "day4/fnmut", source: "day4/_16_fn_pointers/fnmut.rs" },
    Solution { id: "day4/binary_tree", source: "day4/_17_box_pointers/binary_tree.rs" },
    Solution { id: "day4/cons_list", source: "day4/_17_box_pointers/cons_list.rs" },
    Solution { id: "day5/mutex", source: "day5/_18_mutex_pointers/mutex.rs" },
    Solution { id: "day5/rc_shared_data", source: "day5/_19_rc_pointers/rc_shared_data.rs" },
    Solution { id: "day5/rc_smart_pointer", source: "day5/_19_rc_pointers/rc_smart_pointer.rs" },
    Solution { id: "day5/refcell", source: "day5/_20_refcell_pointer/refcell.rs" },
];

enum Action<'a> {
    Help,
    List,
    Run(&'a str),
}

enum RunnerError {
    Usage(String),
    Io(String),
    ProcessFailed {
        operation: &'static str,
        status: ExitStatus,
    },
}

impl RunnerError {
    fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => 2,
            Self::Io(_) => 1,
            Self::ProcessFailed { status, .. } => status.code().unwrap_or(1),
        }
    }
}

impl fmt::Display for RunnerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(message) | Self::Io(message) => formatter.write_str(message),
            Self::ProcessFailed { operation, status } => {
                write!(formatter, "{operation} a echoue ({status})")
            }
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if let Err(error) = run(&arguments) {
        eprintln!("Erreur : {error}");
        process::exit(error.exit_code());
    }
}

fn run(arguments: &[String]) -> Result<(), RunnerError> {
    match parse_action(arguments)? {
        Action::Help => print_help(),
        Action::List => print_solutions(),
        Action::Run(id) => run_solution(id)?,
    }

    Ok(())
}

fn parse_action(arguments: &[String]) -> Result<Action<'_>, RunnerError> {
    match arguments {
        [] => Ok(Action::Help),
        [argument] if matches!(argument.as_str(), "help" | "--help" | "-h") => Ok(Action::Help),
        [command] if command == "list" => Ok(Action::List),
        [id] => Ok(Action::Run(id)),
        _ => Err(RunnerError::Usage(
            "Utilisation : cargo run --bin solutions -- [list|<identifiant>]".to_string(),
        )),
    }
}

fn print_help() {
    println!("Lance une solution de reference sans compiler les exercices etudiants.");
    println!();
    println!("Utilisation :");
    println!("  cargo run --bin solutions -- list");
    println!("  cargo run --bin solutions -- <identifiant>");
    println!();
    println!("Exemple :");
    println!("  cargo run --bin solutions -- day1/variables1");
}

fn print_solutions() {
    println!("Solutions disponibles :");
    for solution in SOLUTIONS {
        println!("  {:<30} {}", solution.id, solution.source);
    }
}

fn run_solution(id: &str) -> Result<(), RunnerError> {
    let solution = find_solution(id).ok_or_else(|| {
        RunnerError::Usage(format!(
            "Solution inconnue : {id}. Utilisez `cargo run --bin solutions -- list`."
        ))
    })?;
    let source = source_path(solution);
    let binary = binary_path(solution);

    let output_directory = binary.parent().expect("the binary path always has a parent");
    fs::create_dir_all(output_directory).map_err(|error| {
        RunnerError::Io(format!(
            "Impossible de creer `{}` : {error}",
            output_directory.display()
        ))
    })?;

    let compilation = Command::new("rustc")
        .arg("--edition")
        .arg("2021")
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .status()
        .map_err(|error| {
            RunnerError::Io(format!(
                "Impossible de lancer rustc pour `{}` : {error}",
                source.display()
            ))
        })?;

    if !compilation.success() {
        return Err(RunnerError::ProcessFailed {
            operation: "La compilation de la solution",
            status: compilation,
        });
    }

    let execution = Command::new(&binary).status().map_err(|error| {
        RunnerError::Io(format!(
            "Impossible d'executer `{}` : {error}",
            binary.display()
        ))
    })?;

    if !execution.success() {
        return Err(RunnerError::ProcessFailed {
            operation: "L'execution de la solution",
            status: execution,
        });
    }

    Ok(())
}

fn find_solution(id: &str) -> Option<&'static Solution> {
    SOLUTIONS.iter().find(|solution| solution.id == id)
}

fn source_path(solution: &Solution) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src/solutions")
        .join(solution.source)
}

fn binary_path(solution: &Solution) -> PathBuf {
    let binary_name = format!(
        "{}{}",
        solution.id.replace('/', "_"),
        env::consts::EXE_SUFFIX
    );

    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/solution-runner")
        .join(binary_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn every_registered_source_exists() {
        for solution in SOLUTIONS {
            assert!(
                source_path(solution).is_file(),
                "missing source for {}",
                solution.id
            );
        }
    }

    #[test]
    fn solution_identifiers_are_unique() {
        let identifiers: BTreeSet<_> = SOLUTIONS.iter().map(|solution| solution.id).collect();
        assert_eq!(identifiers.len(), SOLUTIONS.len());
    }

    #[test]
    fn finds_a_known_solution() {
        let solution = find_solution("day1/variables1").expect("known solution");
        assert_eq!(solution.source, "day1/_01_variables/variables1.rs");
    }

    #[test]
    fn rejects_an_unknown_solution() {
        assert!(find_solution("day9/unknown").is_none());
    }

    #[test]
    fn parses_the_supported_commands() {
        assert!(matches!(parse_action(&[]), Ok(Action::Help)));
        assert!(matches!(parse_action(&["list".to_string()]), Ok(Action::List)));
        assert!(matches!(
            parse_action(&["day1/variables1".to_string()]),
            Ok(Action::Run("day1/variables1"))
        ));
    }

    #[test]
    fn rejects_extra_arguments() {
        let arguments = vec!["list".to_string(), "unexpected".to_string()];
        assert!(matches!(parse_action(&arguments), Err(RunnerError::Usage(_))));
    }
}
