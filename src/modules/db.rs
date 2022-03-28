
use std::collections::HashMap;
use crate::modules::element::*;

pub struct DB{
    map: HashMap<String, Element>,
}

impl DB{
    fn new() -> DB{
        let mut map: HashMap<String,Element> = HashMap::new();
        DB { map }
    }

//    fn set(&self, key: String, value: String){}

}
