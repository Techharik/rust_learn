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
    let char = _greeting.chars().nth(0);
    print!("{}", char.unwrap());

    //    summary - numbers , bool , strings

    //if else

    if is_female {
    } else if is_male {
    } else {
    }

    //loops:
    for i in 0..10 {
        print!("{}", i)
    }

    // iterations //array , maps , strings
    let sentance = "my name is hakirat";

    for i in sentance.chars() {
        print!("{}", i)
    }

    let result = mainer(2, 3);
    print!("{}", result);

    // Memory mangement in rusts:
    // How mameory safe comared to c;
    //
}

fn mainer(a: i32, b: i32) -> i32 {
    return a + b;
}
