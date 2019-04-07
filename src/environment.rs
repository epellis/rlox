use crate::token::Object;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub type Enclosure = Rc<RefCell<HashMap<String, Object>>>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: Enclosure,
    pub enclosure_stack: Vec<Enclosure>,
}

impl Environment {
    pub fn new_root() -> Environment {
        let values = Rc::new(RefCell::new(HashMap::new()));
        let mut enclosure_stack = Vec::new();
        enclosure_stack.push(values.clone());
        Environment { enclosure_stack, values }
    }

    pub fn new_child(parent: &Environment) -> Environment {
        let values = Rc::new(RefCell::new(HashMap::new()));
        let mut enclosure_stack = parent.enclosure_stack.clone();
        enclosure_stack.push(values.clone());
        Environment { enclosure_stack, values }
    }

    pub fn define(&self, name: String, value: Object) {
        let mut enclosure = self.values.borrow_mut();
        enclosure.insert(name, value);
    }

    pub fn get(&self, name: String) -> Object {
        for enclosure in self.enclosure_stack.iter().rev() {
            let enclosure = enclosure.borrow();
            if let Some(object) = enclosure.get(&name) {
                if *object == Object::None {
                    panic!("Variable is nil");
                }
                return object.clone();
            }
        }
        panic!("Get: Undefined Variable!");
    }

    pub fn assign(&self, name: String, object: Object) {
        for enclosure in self.enclosure_stack.iter().rev() {
            let mut enclosure = enclosure.borrow_mut();
            if enclosure.contains_key(&name) {
                enclosure.insert(name, object);
                return;
            }
        }
        panic!("Assign: Undefined Variable!");
    }

//    pub fn globals(&self) -> Option<Enclosure> {
//        *self.enclosure_stack.first()?.clone()
//    }
}

//impl Drop for Environment {
//    fn drop(&mut self) {
//        println!("Enclosure Stack: {:#?}", self.enclosure_stack);
//        println!("Going out of scope: {:#?}", self.values);
//    }
//}
