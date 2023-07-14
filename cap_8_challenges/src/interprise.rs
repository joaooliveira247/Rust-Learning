use std::collections::HashMap;

#[derive(Debug)]
pub struct Interprise {
    users: HashMap<String, Vec<String>>,
}

impl Interprise {
    pub fn new() -> Interprise {
        Interprise {
            users: HashMap::new(),
        }
    }
    pub fn add(&mut self, position: String, name: String) {
        let users_vec = self.users.entry(position).or_default();
        users_vec.push(name);
    }

    pub fn delete(&mut self, position: String, name: String) {
        let user_vec = self.users.entry(position).or_default();
        user_vec.retain(|x| *x != name);
    }

    pub fn read(&mut self) -> HashMap<String, Vec<String>> {
        let ret = self.users.clone();
        ret
    }
}
