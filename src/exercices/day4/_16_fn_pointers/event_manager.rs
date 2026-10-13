// Exercice difficile

pub struct EventManager {
    on_start: Vec<Box<dyn FnMut() -> ()>>,
    on_message: Vec<Box<dyn Fn(String) -> String>>,
    on_shutdown: Vec<Box<dyn FnOnce() -> bool>>,
}

impl EventManager {
    pub fn new() -> Self {
        // TODO() : Initialiser un nouveau gestionnaire d'événements avec
        // des vecteurs vides pour chaque type de gestionnaire
    }

    pub fn register_start_handler(&mut self, handler: impl FnMut() -> () + 'static) {
        // TODO() : Enregistrer un gestionnaire de démarrage qui peut être muté
        // et ne retourne rien (utilise FnMut)
    }

    pub fn register_message_handler(&mut self, handler: impl Fn(String) -> String + 'static) {
        // TODO() : Enregistrer un gestionnaire de message qui peut être appelé
        // plusieurs fois sans mutation (utilise Fn)
    }

    pub fn register_shutdown_handler(&mut self, handler: impl FnOnce() -> bool + 'static) {
        // TODO() : Enregistrer un gestionnaire d'arrêt qui ne peut être appelé
        // qu'une seule fois (utilise FnOnce)
    }

    pub fn trigger_start(&mut self) {
        // TODO() : Déclencher tous les gestionnaires de démarrage enregistrés
    }

    pub fn process_message(&self, message: String) -> Vec<String> {
        // TODO() : Traiter un message avec tous les gestionnaires de message
        // et collecter leurs résultats dans un vecteur
    }

    pub fn shutdown(self) -> bool {
        // TODO() : Appeler tous les gestionnaires d'arrêt et retourner true
        // si tous ont réussi (retourné true), sinon false
    }
}

fn main() {
    // pour tester vos fonctions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handlers_are_called_and_shutdown_combines_their_results() {
        let mut manager = EventManager::new();
        manager.register_start_handler(|| {});
        manager.trigger_start();

        manager.register_message_handler(|message| format!("{}!", message));
        manager.register_message_handler(|message| message.to_uppercase());
        assert_eq!(
            manager.process_message(String::from("hello")),
            vec![String::from("hello!"), String::from("HELLO")]
        );

        manager.register_shutdown_handler(|| true);
        manager.register_shutdown_handler(|| false);
        assert!(!manager.shutdown());
    }
}
