use crate::token::Object;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::Ordering;
use std::fmt;

pub type Enclosure = Rc<RefCell<HashMap<String, Object>>>;

#[derive(Clone)]
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

    pub fn new_from_globals(parent: &Environment) -> Environment {
        let values = Rc::new(RefCell::new(HashMap::new()));
        let mut enclosure_stack = Vec::new();
        enclosure_stack.push(parent.enclosure_stack[0].clone());
        enclosure_stack.push(values.clone());
        Environment { enclosure_stack, values }
    }

    pub fn define(&self, name: String, value: Object) {
        let mut enclosure = self.values.borrow_mut();
        enclosure.insert(name, value);
    }

    pub fn get(&self, name: String) -> Object {
        println!("Get: {}", &name);
        for enclosure in self.enclosure_stack.iter().rev() {
            let enclosure = enclosure.borrow();
            println!("Get(Enclosure): {:?}", &enclosure);
            println!();
            if let Some(object) = enclosure.get(&name) {
                if *object == Object::None {
                    panic!("Variable is nil");
                }
                return object.clone();
            }
        }
        eprintln!("Tried to access: {}", name);
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
}

impl Drop for Environment {
    fn drop(&mut self) {
        println!("Going out of scope: {:#?}", &self.values);
    }
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Environment) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Environment) -> bool {
        false
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Environment: {:?}", self.values)
    }
}
