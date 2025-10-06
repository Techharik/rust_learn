use std::string;

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

    let mut _m: i32 = 23;
    _m = 2; //everything is immutable 
    println!("{}", _m);
    //    everything is imutable we need to say explicitly tell it is mutable.
    //    there can be race condition like if 2 thread try to update the same varible. immutable data is thread-safe
    //    it allows the compiler to make more optimized.

    // stack vs heap allocation
    // Stack neatly  organised vs Heap is clumsy unorganised.
    // some variable are fixed in size but some varibale are not fixed it increases in compile time space.
    // example in str if change the str in runtime how to allocate the memory space
    // heap helps in that cases its a unorganised data , we can put the those datatype in heaps
    // in stack the refernce number is stored and heap the unorganied data will store which size is not fixed;

    //stack - fixed size , number boolean , fixed size array
    //heap  - unfixed size - vector , string;

    // each function has its own stack frame like js all function data in callstack has a context in stack.

    //? so in unfixed data stypes it store the value in the heap and the first index memory reference is stored in the  stack which point to the index of value , and length of the value , capacity how much more can i store in that places.

    // if the capacity is not enough space than it goes to next avaiable space that y the heap is slow.

    let _my_str = String::from("my name is hari prasath");

    println!(
        "{} {} {:p}",
        _my_str.len(),
        _my_str.capacity(),
        _my_str.as_ptr()
    );

    // Ownership - all about the memory mangement . memeory safe risk less.Rules govern
    // we need to follow the rules if we followed the rules the progrom complies if complies than our memory safe is done.after that the memory management is done there.
    // when we pass the value by reference the old is died and and new owner is 2 people cannot point to the same reference.

    let s1 = String::from("Hello");
    let mut s2 = s1; //s1 loss the pointer control

    // another way for move a values;
    s2 = mainer_string(s2); // when we pass the 
    println!("{}", s2);

    // borrwowing and references:
    // multiple people with same time but no other stuff , if you want to do some stuff then 1 single borrow.
    // single owner
    //multiple borrowser stille single owner
    //single owner , one broower do something.

    //    borrow:
    let mut f = String::from("name");
    let f1 = &mut f;
    f1.push_str("he");

    println!("{}", f1);
    println!("{}", f);
    // string_update(&mut f);
    // println!("{}", f);

    // structs. --- like object
}

fn mainer(a: i32, b: i32) -> i32 {
    return a + b;
}

fn mainer_string(a: String) -> String {
    return a;
}

fn string_update(a: &mut String) {
    a.push_str("world");
}
