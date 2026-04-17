use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    println!("Welcome To RUST SMART CLI PROJECT");

    loop {
        println!("Please Enter add/list/exit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        let input = input.trim();

        if input == "exit" {git 
            println!("GoodBye");
            break;
        }

        if input.starts_with("add ") {
            let task = input[4..].to_string();
            tasks.push(task);
            println!("Task Added Successfully");
        } else if input == "list" {
            println!("Your Tasks");
            for (i, task) in tasks.iter().enumerate() {
                println!("{}: {}", i + 1, task);
            }
        } else {
            println!("Unknown Command");
        }
    }
}