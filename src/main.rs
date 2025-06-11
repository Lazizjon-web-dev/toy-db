use rustyline::DefaultEditor;

mod db;

fn main() {
    // Create a new Rustyline editor instance
    let mut rustyline = DefaultEditor::new().unwrap();

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
                line = line.trim().to_string();
                // Process the input line
                match line.split_whitespace().nth(0).unwrap_or("") {
                    "exit" => {
                        println!("Exiting the program.");
                        break;
                    }
                    "help" => {
                        println!("Available commands: \n- exit: Exit the program\n- help: Show this help message");
                    }
                    "get" => {
                        if let Some(key) = line.split_whitespace().nth(1) {
                            match db.get(key) {
                                Some(value) => println!("Value for '{}': {}", key, value),
                                None => println!("No value found for key '{}'", key),
                            }
                        }
                    }
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
