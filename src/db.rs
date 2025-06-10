use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
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
        let contents = from_str::<HashMap<String, String>>(&raw);
        self.value = contents.unwrap_or_default();
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
