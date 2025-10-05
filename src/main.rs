fn main() {
    // simple variable in rusts.
    //numbers in rust i , u , f are memory footprints in rust ;
    let x: i32 = 5;
    let _y: u32 = 100;
    let _z: f64 = 100.001;
    // bool false and true;

    let is_male = true;
    let is_female = false;

    if is_male {
        println!("hi")
    } else {
        println!("hello")
    }

    // strings - does not have fixed size , can change at a runtime.

    let _str = "hello";
    let _greeting = String::from("hello world");

    print!("x:{}", x);
    let char = _greeting.chars().nth(0)
    print!("{}", char);
}
