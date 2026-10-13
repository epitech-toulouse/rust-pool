fn apply_operations<F>(data: &mut Vec<i32>, mut operations: Vec<F>)
where
    F: FnMut(&mut Vec<i32>),
{
    for operation in operations.iter_mut() {
        operation(data);
    }
}

fn main() {
    let mut data = vec![1, 2, 3];
    let operations: Vec<fn(&mut Vec<i32>)> = vec![
        |values| values.push(4),
        |values| values.iter_mut().for_each(|value| *value *= 2),
    ];

    apply_operations(&mut data, operations);
    println!("Donnees apres operations : {:?}", data);
}
