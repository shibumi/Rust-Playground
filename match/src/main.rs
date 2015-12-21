// Import crate (library) for random numbers
extern crate rand;
use rand::Rng;
// Import input/output for reading input
use std::io;

// Function that returns a random unsigned 32bit integer
fn get_random_number() -> u32 {
    rand::thread_rng().gen()
}

// Function that compares two values in a pair
fn compare_numbers(pair : (u32, u32)) -> u32 {
    match pair  {
        (x,y) if x == y => return 0,
        (x,y) if x > y => return 1,
        (x,y) if x < y => return 2,
        _ => return 3,
    }

}

fn main() {
    println!("Please input an unsigned number: ");
    let reader = io::stdin();
    let mut input_text = String::new();
    // Error-handling via ok().expect() in Rust ;)
    // Reads the input and saves it in input_text
    reader.read_line(&mut input_text).ok().expect("failed to read line");
    // Option-Datatype to determine success of unsigned integer parsing
    let input_opt: Option<u32> = input_text.trim().parse::<u32>().ok();
    let input_int = match input_opt {
        Some(input_int) => input_int,
        None            => {
        println!("please input a number");
        return;
        }
    };
    let output : u32 = compare_numbers((input_int, get_random_number()));
    match output {
        0 => println!("Yeah!! You've hit the secret number"),
        1 => println!("Too big..."),
        2 => println!("Too small.."),
        _ => println!("ok.. you shouldn't be here"),
    }
}
