struct EventManager {
    on_start: Vec<Box<dyn FnMut() -> ()>>,
    on_message: Vec<Box<dyn Fn(String) -> String>>,
    on_shutdown: Vec<Box<dyn FnOnce() -> bool>>,
}

impl EventManager {
    fn new() -> Self {
        EventManager {
            on_start: Vec::new(),
            on_message: Vec::new(),
            on_shutdown: Vec::new(),
        }
    }

    fn register_start_handler(&mut self, handler: impl FnMut() -> () + 'static) {
        self.on_start.push(Box::new(handler));
    }

    fn register_message_handler(&mut self, handler: impl Fn(String) -> String + 'static) {
        self.on_message.push(Box::new(handler));
    }

    fn register_shutdown_handler(&mut self, handler: impl FnOnce() -> bool + 'static) {
        self.on_shutdown.push(Box::new(handler));
    }

    fn trigger_start(&mut self) {
        for handler in &mut self.on_start {
            handler();
        }
    }

    fn process_message(&self, message: String) -> Vec<String> {
        let mut vector = Vec::new();

        for handler in &self.on_message {
            let result = handler(message.to_string());
            vector.push(result);
        }
        vector
    }

    fn shutdown(self) -> bool {
        let mut all_success: bool = true;
        for handler in self.on_shutdown {
            if !handler() {
                all_success = false;
            }
        }
        all_success
    }
}

fn main() {
    let mut manager = EventManager::new();
    manager.register_start_handler(|| println!("Gestionnaire de demarrage execute."));
    manager.register_message_handler(|message| format!("Echo : {}", message));
    manager.register_message_handler(|message| message.to_uppercase());
    manager.register_shutdown_handler(|| true);

    manager.trigger_start();
    for response in manager.process_message(String::from("bonjour")) {
        println!("{}", response);
    }
    println!("Arret reussi : {}", manager.shutdown());
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
