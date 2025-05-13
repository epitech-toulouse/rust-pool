// En c pour coder cette fonction vous auriez tendance à coder comme ça.
// Le rust est un langage qui accorde des manières de faire plus simples
// comme dans les lanages fonctionnels.
pub fn sum_even_numbers_like_c(start: i32, end: i32) -> i32 {
    let mut res = 0;
    for i in start..end {
        if i % 2 == 0 {
            res += i;
        }
    }
    res
}

pub fn sum_even_numbers(start: i32, end: i32) -> i32 {
    // TODO() : Implémenter la même fonction que précédemment, mais en utilisant
    // une approche fonctionnelle avec les méthodes d'itérateur (filter, sum)
}

pub fn sum_even_numbers_cpy(numbers: &[i32]) -> i32 {
    // TODO() : Implémenter une fonction qui calcule la somme des nombres pairs
    // dans un tableau en utilisant les méthodes d'itérateur
    // Il y a plusieurs manières de faire...
}

fn main() {
    // pour tester vos fonctions
}
