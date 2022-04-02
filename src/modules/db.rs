use crate::modules::element::*;
use crate::serde::{Deserialize, Serialize};
use crate::serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct DB {
    map: HashMap<String, Element>,
}

impl DB {
    pub fn new() -> DB {
        let map: HashMap<String, Element> = HashMap::new();
        DB { map }
    }

    pub fn set(&mut self, key: &str, value: String) {
        //Check if key already exists
        if self.map.contains_key(key) {
            println!("Key found");
            self.map.get_mut(key).unwrap().get_mut().insert(value);
        } else {
            println!("Inserting new key");
            let element = Element::new(value);
            self.map.insert(key.to_string(), element);
        }
    }

    pub fn get(&self, key: &str) -> &Element {
        
        let values = self.map.get(key).unwrap().get();
        return values;
    }

    pub fn serialize(&self, key: &str) -> Result<()> {
        if !self.map.contains_key(key) {
            //return Err(e);
        }

        let values = serde_json::to_string(&self.map.get(key).unwrap().get())?;
        println!("{}", values);
        Ok(())
        //return String::from("Not implemented yet");
    }

    //
    pub fn get_mut(&mut self, key: &str) -> &mut Element {
        return self.map.get_mut(key).unwrap().get_mut();
    }

    pub fn delete_at_index(&mut self, key: &str, index: usize){
        self.get_mut(key).delete(index);
    }

}
