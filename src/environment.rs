use std::any::Any;
use std::collections::HashMap;
// use crate::Rlox;
use crate::token::{Object, Token};

pub struct Value {
    ty: Object,
    val: Box<dyn Any>,
}

pub struct Environment {
    values: HashMap<String, Value>,
    // rlox: &'a mut Rlox,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, val: Box<dyn Any>, ty: Object) {
        self.values.insert(key, Value { ty, val });
    }

    pub fn get(&mut self, name: Token) -> Box<dyn Any> {
        match self.values.get(&name.get_lexeme()) {
            Some(val) => {
                match val.ty {
                    Object::Number => {
                        let value = *(&*val.val).downcast_ref::<f64>().unwrap();
                        return Box::from(value);
                    }
                    Object::String => {
                        let value = (&*val.val).downcast_ref::<String>().unwrap();
                        return Box::new(value.clone());
                    }
                    Object::Nil => {
                        return Box::from(Option::<String>::None);
                    }
                    Object::Bool => {
                        // Should never store a boolean
                        unreachable!()
                    }
                }
            }
            None => {
                // self.rlox.runtime_error(name.get_line(), format!("Undefined Variable: {:?}", name.get_lexeme()));
                return Box::from(Option::<String>::None);
            }
        }
    }
}
