fn main() {
    println!("----- String and slices and others ------ ");
    let str = String::from("Hello world ®️");
    let str_slice = &str[0..2];

    let find_first_word = find_first(&str);

    println!("{}", find_first_word); // find the first word as reference in the owner string
    println!("{}", str); // owner
    println!("{}", str_slice); // reference of the sliced owner;

    //Generic Videos:

    fn generic_fun<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
        if a > b {
            return a;
        }
        return b;
    }

    // Traits : kind of interface in ts or class simply put:
    // like blue pring when struct impl the trait then the trait has to add the function

    pub trait Summary {
        //implement all the methods that we want here and the function impl it must have the things we can use the default for some defaul sending but can't access the self.property inside it some work around there.

        fn summerize(&self) -> String;
    }

    struct User {
        name: String,
        age: i32,
    }

    impl Summary for User {
        fn summerize(&self) -> String {
            return format!("{}, {}", self.name, self.age);
        }
    };

    let user = User {
        name: String::from("Hari"),
        age: 22,
    };

    println!("{}", user.summerize())
}

fn find_first(str: &String) -> &str {
    let mut index = 0;

    for i in str.chars() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }

    return &str[0..index];
}
