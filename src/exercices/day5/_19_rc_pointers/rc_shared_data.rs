use std::rc::Rc;

pub fn create_shared_resource() -> Rc<Vec<i32>> {
    // TODO() : Créer un vecteur d'entiers et le transformer en une ressource
    // partagée en l'enveloppant dans un Rc (Reference Counted smart pointer)
}

pub fn add_consumer(resource: &Rc<Vec<i32>>) -> usize {
    // TODO() : Créer un nouveau consommateur en clonant le Rc
    // (ce qui incrémente le compteur de références)
    // Retourner le nombre actuel de références fortes
}

fn main() {
    // pour tester vos fonctions
}

#[test]
fn test_shared_resource() {
    let original_vec = create_shared_resource();

    let _consumer1 = Rc::clone(&original_vec);
    assert_eq!(Rc::strong_count(&original_vec), 2);

    let _consumer2 = Rc::clone(&original_vec);
    assert_eq!(Rc::strong_count(&original_vec), 3);
}
