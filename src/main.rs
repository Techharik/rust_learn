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

    let _str: &'static str = "hello";
    let _greeting: String = String::from("hello world");

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
    println!("{}", result);

    // Memory mangement in rusts:
    // How mameory safe comared to c;
    // js , java - uses Garbage collections it do automatic collection , we dont access the memory mangement
    // c++ manual memory mangement , allcoate and dealocate in c , you deal with ram memory
    // when we do the manual memory collection we go in dangling issue lead to runtime issues.Learning curve is high.
    // rust way . rust has its own ownership model for memory management.make it extremely safe but follow the rules.

    // it achive with mutablity , heap and stacks , ownership , borrowing and reference , lifetimes.

    // mutability?
    // -- immutable varibale is cannot change those varibales once assigned. like const in js.

    // mutable - values are changeable.

    let mut m: i32 = 23;
    m = 2; //everything is immutable 
    println!("{}", m)
    //    everything is imutable we need to say explicitly tell it is mutable.
    //    there can be race condition like if 2 thread try to update the same varible. immutable data is thread-safe
    //    it allows the compiler to make more optimized.

    // stack vs heap allocation
    // Stack neatly  organised vs Heap is clumsy unorganised.
    // some variable are fixed in size but some varibale are not fixed it increases in compile time space.
    //
}

fn mainer(a: i32, b: i32) -> i32 {
    return a + b;
}
