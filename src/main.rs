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
                match line.trim() {
                    "exit" => {
                        println!("Exiting the program.");
                        break;
                    }
                    "help" => {
                        println!("Available commands: \n- exit: Exit the program\n- help: Show this help message");
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
