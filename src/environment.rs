use crate::token::Object;
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Object {
        self.values.get(&name).expect("Undefined Variable").clone()
    }

    pub fn assign(&mut self, name: String, value: Object) {
//        if self.values.insert(name, value).is_none() {
//            panic!("Undefined variable");
//        }
        self.values.insert(name, value).expect("Undefined Variable");
    }
}
