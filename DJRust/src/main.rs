extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number !");
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your Guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    foo();
    let mut s = "shello.txt".to_string();
    file_read(&mut s);
    //let x = 5;
    //let y = 10;
    // println!("x and y: {} and {}", x, y);
}

fn foo(){
    println!("Yes I am a function ");
}

fn file_read(s: &mut String) {
    // Create a path to the desired file
    let path = Path::new(s);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}