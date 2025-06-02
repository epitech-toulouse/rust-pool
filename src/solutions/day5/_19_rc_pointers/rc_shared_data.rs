use std::rc::Rc;

fn create_shared_resource() -> Rc<Vec<i32>> {
    let my_vec = vec![1, 2, 3, 4, 5];
    Rc::new(my_vec)
}

fn main() {
    let original_vec = create_shared_resource();
    println!(
        "Nombre initial de références: {}",
        Rc::strong_count(&original_vec)
    );

    let consumer1 = Rc::clone(&original_vec);
    println!(
        "Après 1er consommateur: {}",
        Rc::strong_count(&original_vec)
    );

    let consumer2 = Rc::clone(&original_vec);
    println!(
        "Après 2ème consommateur: {}",
        Rc::strong_count(&original_vec)
    );

    let consumer3 = Rc::clone(&original_vec);
    println!(
        "Après 3ème consommateur: {}",
        Rc::strong_count(&original_vec)
    );

    {
        let _temp_consumer = Rc::clone(&original_vec);
        println!(
            "Dans le bloc avec référence temporaire: {}",
            Rc::strong_count(&original_vec)
        );
    }

    println!("Après la fin du bloc: {}", Rc::strong_count(&original_vec));
    println!("Contenu du vecteur partagé: {:?}", *original_vec);
}

#[test]
fn test_shared_resource() {
    let original_vec = create_shared_resource();
    assert_eq!(Rc::strong_count(&original_vec), 1);

    let consumer1 = Rc::clone(&original_vec);
    assert_eq!(Rc::strong_count(&original_vec), 2);

    let consumer2 = Rc::clone(&original_vec);
    assert_eq!(Rc::strong_count(&original_vec), 3);
    assert_eq!(*original_vec, vec![1, 2, 3, 4, 5]);
}
