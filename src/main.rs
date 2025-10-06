fn main() {
    println!("{}", "My rust revision here");
    // fibnocii seq algo in using rust;

    let fibseq = fib(5);

    let str = String::from("Hello h");
    let length_str = str_len(&str);

    println!("{}", str);
    println!("{}", length_str);
    println!("{}", fibseq);
    println!("{}", fibser(3))
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
