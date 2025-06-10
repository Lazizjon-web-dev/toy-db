use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub struct Database {
    pub value: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            value: HashMap::new(),
        }
    }

    pub fn initialize_from_file(&mut self, file_path: &str) -> Result<(), String> {
        let raw = fs::read_to_string(file_path).expect("Should have been able to read the file");
        let contents = serialze::<HashMap<String, String>>(raw);
    }
}
