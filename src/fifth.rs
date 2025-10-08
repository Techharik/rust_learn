use std::io;
fn main() {
    println!("Macros and other things");

    println!("-------");

    println!("Rev - 2");
    let a = 32;

    // Data types - scalar types  - intergers , flotings, numbers , char
    println!("{}", a);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
