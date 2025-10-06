use std::vec;

fn main() {
    println!("Advance Rust");
    // collections unFixed sizes;

    // vector a dynamic array;

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(3);
    vec.push(4);
    let even = is_even(&vec);
    println!("{:?}", vec);
    println!("{:?}", even)
}

fn is_even(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for &i in vec {
        if i % 2 == 0 {
            new_vec.push(i);
        }
    }
    return new_vec;
}
