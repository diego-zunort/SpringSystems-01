use std::io;
use std::process::Command;

enum FileOperation{
        List(String),
        Display(String),
        Create(String,String),
        Remove(String),
        Pwd,
    }


fn main() {

    println!("Welcome to the File Operation Program!");
    
    loop{
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("\nEnter your choice (0-5):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        let operation = match choice {
            "1" => {
                println!("Enter directory path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                FileOperation::List(path.trim().to_string())
            }

            "2" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                FileOperation::Display(path.trim().to_string())
            }

            "3" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                println!("Enter content:");
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                FileOperation::Create(path.trim().to_string(), content.trim().to_string())
            }

            "4" => {
                println!("Enter file path:");
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                FileOperation::Remove(path.trim().to_string())
            }

            "5" => FileOperation::Pwd,

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please try again.");
                continue;
            }
        };

        perform_operation(operation);
    }
}

fn perform_operation(operation: FileOperation) {
    match operation {

        FileOperation::List(directory_path) => {
            let status = Command::new("ls")
            .arg(directory_path)
            .status()
            .expect("Failed to execute ls");

            if !status.success() {
                println!("Error listing directory.");
            }
        }

        FileOperation::Display(file_path) => {
            let status = Command::new("cat")
                .arg(file_path)
                .status()
                .expect("Failed to execute cat");

            if !status.success() {
                println!("Error displaying file.");
            }
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if status.success() {
                println!("File '{}' created successfully.", file_path);
            } else {
                println!("Failed to create file.");
            }
        }

        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(file_path)
                .status()
                .expect("Failed to execute rm");

            if status.success() {
                println!("File removed successfully.",);
            } else {
                println!("Failed to remove file.");
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if !status.success() {
                println!("Error retrieving working directory.");
            }
        }
    }
}


