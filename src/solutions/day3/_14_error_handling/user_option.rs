use std::u8;

pub struct User {
    username: String,
    email: Option<String>,
    age: Option<u8>,
}

impl User {
    pub fn new(username: String) -> Self {
        User {
            username,
            email: None,
            age: None,
        }
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn with_age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    pub fn get_contact_info(&self) -> String {
        match &self.email {
            Some(email) => format!("{} ({})", self.username, email),
            None => format!("{} (email not provided)", self.username),
        }
    }

    pub fn is_adult(&self) -> Option<bool> {
        self.age.map(|age| age >= 18)
    }
}

fn main() {
    let user = User::new(String::from("alice"))
        .with_email(String::from("alice@example.com"))
        .with_age(25);

    println!("Contact : {}", user.get_contact_info());
    println!("Est adulte : {}", user.is_adult().unwrap_or(false));
}
