use std::thread;

fn main() {
    println!("LifeTimes in Rusts");
    let long_live;

    let str1 = String::from("small");

    let str2 = String::from("longest");

    long_live = longer_fun(&str1, &str2);

    //   multi theadring:
    let v = vec![1, 2, 3, 32, 43, 43, 54, 534];
    let ans = thread::spawn(move || {
        println!("Inside thread!");
        let new_vec: Vec<i32> = v.iter().map(|&x| x * 2).collect();
        new_vec
    });
    println!("{}", long_live);

    // Wait for the thread to finish and get the result
    let new_vec = ans.join().unwrap();
    println!("Thread result: {:?}", new_vec);
}

fn longer_fun<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}
