use core::num;
use std::{char, fs};

fn main() {
    println!("{}", "My rust revision here");
    // fibnocii seq algo in using rust;

    let fibseq = fib(5);

    let str = String::from("Hello h");
    let length_str = str_len(&str);

    let user1 = User {
        is_active: false,
        username: String::from("Hari"),
        sign_in_count: 34,
    };

    println!("{}", user1.greet());
    println!("{}", user1.username);
    // println!("{}", str);

    // println!("{}", length_str);
    // println!("{}", fibseq);
    // println!("{}", fibser(3))

    //enums - fixed variants , exceptional handlings , null handlings;

    enum Shape {
        Cicle(f64),
    }

    let shape = Shape::Cicle(3.00);

    fn create_shape(shape: Shape) -> f64 {
        match shape {
            Shape::Cicle(val) => val * 10.00,
        }
    }

    println!("{}", create_shape(shape));

    // result and optionals in enums for exceptional handling and null pointers:
    match is_even(3) {
        Some(value) => println!("{}", value + 10),
        None => println!("{}", "There is no value present"),
    }

    let read_file = fs::read_to_string("./src/first.rs");

    match read_file {
        Ok(cont) => println!("Hello This is the contant in the files {}", cont),
        Err(err) => println!("Error {}", err),
    }
}

fn is_even(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        return Some(num);
    }
    return None;
}

struct User {
    is_active: bool,
    username: String,
    sign_in_count: u64,
}

impl User {
    fn greet(&self) -> String {
        let message = String::from("Hello ");
        message + &self.username
    }

    //static function kind of without the & self
    fn Debug() -> u32 {
        return 1;
    }
}

// fib - serious;
fn fib(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }

    return num * fib(num - 1);
}

fn fibser(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        println!("I am printing");
        let temp = second;
        second = second + first;
        first = temp
    }
    return second;
}

fn str_len(str: &String) -> usize {
    return str.chars().count();
}
