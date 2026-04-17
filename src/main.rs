use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    println!("Welcome To RUST SMART CLI PROJECT");

    loop {
        println!("Please Enter add/list/delete/exit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        let input = input.trim();

        if input == "exit" {
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

        } else if input.starts_with("delete") {
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.len() != 2 {
                println!("Usage: delete <number>");
                continue;
            }

            match parts[1].parse::<usize>() {
                Ok(num) => {
                    if num == 0 || num > tasks.len() {
                        println!("Invalid index");
                    } else {
                        tasks.remove(num - 1);
                        println!("Task deleted, available Tasks");
                        for (i, task) in tasks.iter().enumerate() {
                            println!("{}: {}", i + 1, task);
                        }
                    }
                }
                Err(_) => println!("Invalid number"),
            }

        } else {
            println!("Unknown Command");
        }
    }
}