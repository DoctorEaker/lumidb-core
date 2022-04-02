use crate::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Element {
    values: Vec<String>,
    version: u32,
}

impl Element {
    pub fn new(value: String) ->  Element{
        let mut values: Vec<String> = Vec::new();
        values.push(value);
        let version = 0;
        Element {
            values,
            version,
        }
    }

    pub fn insert(&mut self, value:String){
        &self.values.push(value);
    }

    pub fn get(&self) ->  &Element{
        return self;
    }

    pub fn get_mut(&mut self) -> &mut Element{
        return self;
    }

    pub fn get_values(&self) -> Vec<String>{
        let values = self.values.clone();

        //println!("Getting all values asociated with a key");

        return values;
    }

    pub fn delete(&mut self, index:usize){
        self.values.remove(index);
    }
}
