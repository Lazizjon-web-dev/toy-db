use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    HashMap(HashMap<String, Self>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub value: Vec<HashMap<String, Value>>,
    pub path: Option<String>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            value: Vec::new(),
            path: None,
        }
    }

    pub fn initialize_from_file(&mut self, file_path: &str) -> Result<(), String> {
        let raw = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let contents = from_str::<Vec<HashMap<String, Value>>>(&raw);
        self.value = contents.unwrap_or_default();
        self.path = Some(String::from(file_path));
        Ok(())
    }

    pub fn query(&self, key: &str, value: Value) -> Option<&HashMap<String, Value>> {
        println!("Checking for self.value: {:?}", self.value);
        // Iterate through each map in the vector
        for map in self.value.iter() {
            // Check if the map contains the key and if its value matches the provided value
            if let Some(v) = map.get(key)
                && v == &value
            {
                // If a match is found, return a reference to the map
                return Some(map);
            }
        }
        // If no match is found, return None
        None
    }

    pub fn insert(&mut self, map: HashMap<String, Value>) {
        // Insert the new map into the vector
        self.value.push(map);
        // Save the updated database to the file
        if let Err(e) = self.save() {
            eprintln!("Error saving database: {}", e);
        }
    }

    pub fn remove(&mut self, key: &str, value: Value) -> Result<(), String> {
        // Find the index of the map that contains the key-value pair
        if let Some(index) = self
            .value
            .iter()
            .position(|map| map.get(key) == Some(&value))
        {
            // Remove the map at the found index
            self.value.remove(index);
            // Save the updated database to the file
            if let Err(e) = self.save() {
                eprintln!("Error saving database: {}", e);
            }
            Ok(())
        } else {
            Err(format!("Key '{}' with value '{:?}' not found", key, value))
        }
    }
    // pub fn get(&mut self, key: &str) -> Option<String> {
    //     self.value.get(key).cloned()
    // }

    // pub fn set(&mut self, key: &str, value: &str) {
    //     self.value.insert(key.to_string(), value.to_string());
    //     if let Err(e) = self.save() {
    //         eprintln!("Error saving database: {}", e);
    //     }
    // }

    // pub fn remove(&mut self, key: &str) -> Result<String, String> {
    //     let removed_value = self.value.remove(key);
    //     if let Err(e) = self.save() {
    //         eprintln!("Error saving database: {}", e);
    //     }
    //     removed_value.ok_or_else(|| format!("Key '{}' not found", key))
    // }

    fn save(&self) -> Result<(), Error> {
        match &self.path {
            Some(path) => fs::write(path, to_string_pretty(&self.value).unwrap())?,
            None => {}
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_initialization() {
        let mut db = Database::new();
        assert!(db.value.is_empty());
        let path = "test_db.json";
        db.initialize_from_file(path).unwrap();

        assert_eq!(db.value.get("name").unwrap(), &String::from("Alan"))
    }
}
