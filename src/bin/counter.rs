
use std::io;



fn handle_input(input: &str, count: &mut i32) -> bool {
    match input {
        "+" => {
            *count += 1;
            println!("count: {}", count);
        },
        "-" => {
            *count -= 1;
            println!("count: {}", count);
        },
        "reset" =>{
             *count = 0;
             println!("count: {}", count);
        },
        "show" => {
            println!("Current count: {}", count);
        },
        "exit" => {
            return false;
        },
        _ => {
            println!("Invalid input");
        },
    }
    true
}


pub fn main() {
    let mut count = 0;

    loop {
        println!("Please enter +/-/exit");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read Input");

        let input = input.trim();
        let continue_loop = handle_input(input, &mut count);

        if !continue_loop {
            println!("Exited");
            break;
        }
    }
}

