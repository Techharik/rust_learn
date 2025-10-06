fn main() {
    println!("{}", "My rust revision here");
    // fibnocii seq algo in using rust;

    let fibseq = fib(5);

    println!("{}", fibseq)
}

fn fib(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }

    return num * fib(num - 1);
}
