#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidNumber,
    OutOfRange,
}

fn parse_age(input: &str) -> Result<u8, ParseError> {
    // TODO() : Implémenter une fonction qui analyse une chaîne de caractères
    // et la convertit en âge (u8). La fonction doit :
    // - Retourner ParseError::EmptyInput si la chaîne est vide
    // - Retourner ParseError::InvalidNumber si la chaîne n'est pas un nombre valide
    // - Retourner ParseError::OutOfRange si le nombre est supérieur à 120
    // - Retourner Ok(age) si l'âge est valide (entre 0 et 120)
}

fn main() {
    // pour tester
}
