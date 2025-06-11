use db::Database;
use rustyline::DefaultEditor;
use std::env;

mod db;

fn main() {
    // Create a new Rustyline editor instance
    let mut rustyline = DefaultEditor::new().unwrap();
    let args: Vec<String> = env::args().collect();

    let file_path = &args.get(1).expect("File path argument is required");
    // Initialize the Database
    let mut db = Database::new();

    db.initialize_from_file(file_path)
        .expect("Failed to initialize database from file");
    println!("Database initialized from file: {}", file_path);

    loop {
        let readline = rustyline.readline(">> ");
        match readline {
            Ok(line) if line.trim() == "" => {
                // If the line is empty, continue to the next iteration
                println!("Empty input, please enter a command.");
                continue;
            }
            Ok(line) => {
                // Trim whitespace from the input line
                let line = line.trim().to_string();
                // Process the input line
                match line.split_whitespace().nth(0).unwrap_or("") {
                    "exit" => {
                        println!("Exiting the program.");
                        break;
                    }
                    "help" => {
                        println!("Available commands: \n- exit: Exit the program\n- help: Show this help message");
                    }
                    "query" => {
                        let mut parts = line.split_whitespace();
                        if let (Some(key), Some(value)) = (parts.nth(1), parts.next()) {
                            match db.query(key, value) {
                                Some(v) => println!("Found value: {:?}", v),
                                None => println!("No matching value found for key '{}'", key),
                            }
                        } else {
                            eprintln!("Usage: query <key> <value>");
                        }
                    }
                    // "get" => {
                    //     if let Some(key) = line.split_whitespace().nth(1) {
                    //         match db.get(key) {
                    //             Some(value) => println!("Value for '{}': {}", key, value),
                    //             None => println!("No value found for key '{}'", key),
                    //         }
                    //     }
                    // }
                    // "set" => {
                    //     let mut parts = line.split_whitespace();
                    //     if let (Some(key), Some(value)) = (parts.nth(1), parts.next()) {
                    //         db.set(key, value);
                    //         println!("Set value for '{}': {}", key, value);
                    //     }
                    // }
                    // "remove" => {
                    //     if let Some(key) = line.split_whitespace().nth(1) {
                    //         match db.remove(key) {
                    //             Ok(value) => {
                    //                 println!("Removed key '{}', value was: {}", key, value)
                    //             }
                    //             Err(err) => eprintln!("Error removing key '{}': {}", key, err),
                    //         }
                    //     }
                    // }
                    _ => {
                        // Handle other commands here
                        println!("You entered: {}", line);
                    }
                }
            }
            Err(err) => {
                // Handle errors, such as EOF or other input errors
                println!("Error reading input: {}", err);
            }
        }
    }
}
