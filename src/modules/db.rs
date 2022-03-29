use std::collections::HashMap;
use crate::serde::{Deserialize, Serialize};
use crate::serde_json::Result;
use crate::modules::element::*;

#[derive(Serialize, Deserialize)]
pub struct DB{
    map: HashMap<String, Element>,
}

impl DB{
    pub fn new() -> DB{
        let mut map: HashMap<String,Element> = HashMap::new();
        DB { map }
    }

    pub fn set(&mut self, key: &str, value: String){
        //Check if key already exists
        if self.map.contains_key(key){
            println!("Key found");
        } else {
            println!("Inserting new key");
            let mut element = Element::new(value);
            self.map.insert(key.to_string(),element);
        }
    }

    pub fn get(&self, key: &str) -> Result<()>{
        if !self.map.contains_key(key){
     //       return String::from("Key: ") + key + " not found";
        }

        let values = serde_json::to_string(&self.map.get(key).unwrap().get())?;
        println!("{}",values);
        Ok(())
        //return String::from("Not implemented yet");
    }

}
