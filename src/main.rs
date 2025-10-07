use std::collections::HashMap;
use std::vec;
fn main() {
    println!("Advance Rust");
    // collections unFixed sizes;

    // vector a dynamic homogenous array; unfixed size with same typed values collections

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(3);
    vec.push(4);
    let even = is_even(&vec);
    println!("{:?}", vec);
    println!("{:?}", even);

    // HashMaps - like a obj in js key value pairs in rusts.

    // insert, get , remove, clear:

    let mut users = HashMap::new();

    users.insert(String::from("name"), String::from("hari"));

    let get_users = users.get("name");
    match get_users {
        Some(val) => println!("{}", val),
        None => println!("user is not found"),
    }

    //    Iterators - looping throw the collection types.
    let v1 = vec![1, 2, 3, 4, 5];

    let v1_iter = v1.iter(); //underthe hood this happens.

    println!("{:?}", v1_iter);
    // import thing here the iter() is just borrowed the values means it dont have the ownerships
    //so we cant update the owner vec;
    for val in v1_iter {
        println!("{}", val)
    }

    //if we want to take the mutable reference use iter_mut and update the owner array;

    let mut v2 = vec![1, 2, 3];

    for value in v2.iter_mut() {
        *value *= *value
    }
    println!("{:?}", v2);
    //if want to take the ownerships from a iterator:

    let mut v3 = vec![1, 2, 3];

    let own = v3.into_iter(); //ownership is moved so v3 is not in mmeory

    for v in own {
        v * 1;
    }
    // print!("{:?}", own) //after the loop the iteraton becomes empty so

    let v4 = vec![1, 2, 3];

    for value in v4 {
        println!("{}", value);
    }
    // print!("v4{:?}", v4);  - it wont work because under the hood it uses into.iter()

    // cunsuming adapters:
    let v5 = vec![1, 2, 3, 4];

    let v5_iter = v5.iter();

    let total: i32 = v5_iter.sum();

    println!("{:?}", v5);
    println!("{:?}", total);
    // println!("{:?}", v5_iter) //value is borrowed afer the sum

    // Do a operation and return a  new array;
    let v6 = vec![1, 2, 3, 4]; //initalize the vec

    let iter = v6.iter(); // borrowed the reference of the value

    let iter2 = iter.map(|x| x * 2); //with borrowed referenece create a new iter vec

    for x in iter2 {
        //by looping we doo under the hood we use into.iter so the ownership is borrowed
        println!("{:?}", x)
    }

    // println!("{:?}", iter2) //we can use it here
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
