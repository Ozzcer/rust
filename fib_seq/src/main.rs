use std::{io, process::exit};

fn main() {
    let mut input: String = String::new();

    println!("Please enter the nth term of the fib seq you'd like to calculate");

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number!");
            exit(0);
        },
    };

    let mut fib_previous: u32 = 0;
    let mut fib_current: u32 = 1;

    println!("{fib_previous}");
    for _i in 1..input {
        println!("{fib_current}");
        let fib_next: u32 = fib_current + fib_previous;
        fib_previous = fib_current;
        fib_current = fib_next;
    }

    println!("The {input}th term is {fib_current}");
}
