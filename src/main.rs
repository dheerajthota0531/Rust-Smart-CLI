use std::fs;
use std::io;

use std::sync::mpsc::{self, Sender};
use std::thread;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}
#[derive(Serialize, Deserialize, Debug)]
enum WriteCommand {
    Save(Vec<Task>),
    Quit,
}

fn save_tasks(tx: &Sender<WriteCommand>, tasks: &[Task]) {
    tx.send(WriteCommand::Save(tasks.to_vec())).unwrap();
}

fn load_tasks() -> Vec<Task> {
    match fs::read_to_string("tasks.json") {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(_) => Vec::new(),
    }
}

fn main() {
    let (tx, rx) = mpsc::channel::<WriteCommand>();
    let writer_handle = thread::spawn(move || {
        for cmd in rx {
            match cmd {
                WriteCommand::Save(tasks) => {
                    let data = serde_json::to_string_pretty(&tasks).unwrap();
                    fs::write("tasks.json", data).unwrap();
                }
                WriteCommand::Quit => break,
            }
        }
    });

    let mut tasks = load_tasks();

    println!("Welcome To RUST SMART CLI PROJECT ");
    loop {
        println!("\nCommands: add <task> | list | delete <id> | done <id> | exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");

        let input = input.trim();

        if input == "exit" {
            save_tasks(&tx, &tasks);
            tx.send(WriteCommand::Quit).unwrap();
            writer_handle.join().unwrap();
            println!("Goodbye 👋");
            break;
        } else if input.starts_with("add ") {
            let title = input[4..].to_string();

            let new_task = Task {
                id: tasks.len() as u32 + 1,
                title,
                completed: false,
            };

            tasks.push(new_task);
            save_tasks(&tx, &tasks);

            println!("Task added ");
        } else if input == "list" {
            if tasks.is_empty() {
                println!("No tasks found");
                continue;
            }

            println!("\nYour Tasks:");
            for task in &tasks {
                println!(
                    "{}. [{}] {}",
                    task.id,
                    if task.completed { "x" } else { " " },
                    task.title
                );
            }
        } else if input.starts_with("delete ") {
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.len() != 2 {
                println!("Usage: delete <id>");
                continue;
            }

            match parts[1].parse::<usize>() {
                Ok(num) => {
                    if num == 0 || num > tasks.len() {
                        println!("Invalid id");
                    } else {
                        tasks.remove(num - 1);

                        for (i, task) in tasks.iter_mut().enumerate() {
                            task.id = i as u32 + 1;
                        }

                        save_tasks(&tx, &tasks);
                        println!("Task deleted ");
                    }
                }
                Err(_) => println!("Invalid number"),
            }
        } else if input.starts_with("done ") {
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.len() != 2 {
                println!("Usage: done <id>");
                continue;
            }

            match parts[1].parse::<usize>() {
                Ok(num) => {
                    if num == 0 || num > tasks.len() {
                        println!("Invalid id");
                    } else {
                        tasks[num - 1].completed = true;
                        save_tasks(&tx, &tasks);
                        println!("Task marked as completed ");
                    }
                }
                Err(_) => println!("Invalid number"),
            }
        } else {
            println!("Unknown command ");
        }
    }
}
