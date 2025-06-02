use std::rc::Rc;

pub fn create_shared_resource() -> Rc<Vec<i32>> {
    Rc::new(vec![1, 2, 3, 4, 5])
}

pub fn add_consumer(resource: &Rc<Vec<i32>>) -> usize {
    let _consumer = Rc::clone(resource);
    Rc::strong_count(resource)
}

fn main() {
    let shared = create_shared_resource();
    println!(
        "Nombre initial de références : {}",
        Rc::strong_count(&shared)
    );

    let mut consumers = Vec::new();

    consumers.push(Rc::clone(&shared));
    println!("Après 1er consommateur : {}", Rc::strong_count(&shared));

    consumers.push(Rc::clone(&shared));
    println!("Après 2ème consommateur : {}", Rc::strong_count(&shared));

    consumers.push(Rc::clone(&shared));
    println!("Après 3ème consommateur : {}", Rc::strong_count(&shared));
}

#[test]
fn test_shared_resource() {
    let original_vec = create_shared_resource();

    let _consumer1 = Rc::clone(&original_vec);
    assert_eq!(Rc::strong_count(&original_vec), 2);

    let _consumer2 = Rc::clone(&original_vec);
    assert_eq!(Rc::strong_count(&original_vec), 3);
}
