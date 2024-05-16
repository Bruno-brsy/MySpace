// PRELUDE:   (it's import the standard library basicly)
// The prelude is the list of things that Rust automatically imports into every Rust program.
// It's kept as small as possible, and is focused on things, particularly traits,
// which are used in almost every single Rust program.

// create (in rust) = library / package
// inside a create, there can be multiple module


use std::io;  // import the module io from the create std

fn main() {
    tutorial8();
    not_used();
}

fn not_used() {
    println!("");
    println!("");
    println!("TUTORAL 2 and 3");
    tutorial2_3();

    println!("");
    println!("");
    println!("TUTORAL 4");    
    const SECOND_IN_MINUTE: u32 = 60;
    // const SECOND_IN_MINUTE: u32 = 100; // error, SECOND_IN_MINUTE is a const
    tutorial4(SECOND_IN_MINUTE);
    
    println!("");
    println!("");
    println!("TUTORAL 5");
    tutorial5();
    
    println!("");
    println!("");
    println!("TUTORAL 6");
    tutorial6();

    println!("");
    println!("");
    println!("TUTORLA 7");
    tutorial7();
}

fn tutorial2_3() {
    println!("Hello, world!");
    let mut x;  // mutable, by default, it's unmutable  (what the diff with CONST??)
    x = 4;
    println!("X is: {}", x);
    x += 2;
    println!("X + 2 is {}", x);
    let x = 6; // redefine x  // by default, it's i32 (it's signed)
    println!("new X is: {}", x);
    {
        // new scope
        let x = x + 10; // can use the x from the outer scope, but if we defined a new one, we overwrite it (only for that scope)
        println!("X is: {}", x);
    }
    println!("new X is: {}", x);
    // x = "hello"; // error because, x should be a int
}

fn tutorial4(_sec_in_min: u32) {
    let true_or_false: bool = true; // false
    println!("{}", true_or_false);
    let letter: char = 'd';
    println!("{}", letter);

    let mut tup: (i32, bool, char) = (1, true, 's'); // need to make the tuple mutable to be able to modify the first value
    let tup2: (i8, bool, char) = (1, true, 's'); // not the same type

    println!("{}", tup.0);
    println!("{}", tup2.0);

    tup.0 = 4;
    println!("{}", tup.0);

    // tup = (1, true, 's', 10) // not possible, because it will change the type of the tupple

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];  // should have the same type of element inside, and can't add element into the array
    println!("{}", arr[0]);
    arr[0] = 10;
    println!("{}", arr[0]);

    // let mut arr: [i32; 5];
    // println!("{}", arr[4]); // should initiate the array before access it

    let x: u8 = 4;
    let y = x; // the y will be a u8
    println!("{}, {}", x, y);
}


fn tutorial5() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line"); // get the input from the std imput; read_line take String as input
    // by default, it passing a copy of a value, not the actual value, so we are using a reference (a ref is unmutable by default)
    // expect catch all the error that read_line can throw
    // if we don't put the expect:
    //
    // warning: unused `Result` that must be used
    //   --> src\main.rs:71:5
    //    |
    // 71 |     io::stdin().read_line(&mut input); // get the input from the std imput
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    //    = note: this `Result` may be an `Err` variant, which should be handled
    //    = note: `#[warn(unused_must_use)]` on by default
    // help: use `let _ = ...` to ignore the resulting value
    //    |
    // 71 |     let _ = io::stdin().read_line(&mut input); // get the input from the std imput
    //    |     +++++++

    println!("{}", input);
}

fn tutorial6() {
    let x: u8 = 12; //    0 -> 255
    let y: i8 = 10; // -128 -> 127
    // let f: f32 = 10; // not possible ->>  expected `f32`, found integer;  help: use a float literal: `10.0`
    let f: f32 = 10.0;
    println!("{}, {}, {}", x, y, f);

    // let z = x + y; // error, mismatched type



    let x: u8 = 255; //    0 -> 255
    let y: u8 = 10;  //    0 -> 255
    // let z = x + y;   // overflow
    // // error: this arithmetic operation will overflow
    // //    --> src\main.rs:104:13
    // //     |
    // // 104 |     let z = x + y;
    // //     |             ^^^^^ attempt to compute `u8::MAX + 10_u8`, which would overflow
    // //     |
    // //     = note: `#[deny(arithmetic_overflow)]` on by default


    // let z:u16 = x + y; // mismatched types
    // // error[E0308]: mismatched types
    // //    --> src\main.rs:104:18
    // //     |
    // // 104 |     let z: u16 = x + y;
    // //     |            ---   ^^^^^ expected `u16`, found `u8`
    // //     |            |
    // //     |            expected due to this
    // //     |
    // // help: you can convert a `u8` to a `u16`
    // //     |
    // // 104 |     let z: u16 = (x + y).into();         // /!\ CAN'T DO THAT EITHER, BECAUSE x + y return a u8 in any case
    // //     |                  +     ++++++++
    let z = x / y;  // return a 25

    println!("{}", z);

    let x: f32 = 259.312; // all good (it's set 259.312 has a f32)
    let y      = 10f32;  // all good (it's set 10 has a f32)
    let z      = x % y; // modulo
    println!("{}", z);

    let x = 127_000 as i64;
    let y = 10_i64;
    let z = x / y;
    println!("{}", z);

    // manual convertion, it's good to go from the smaller type to larger type
    let x: i64 = (i32::MAX as i64) + 1;  // take the max number for i32 and store it has i64, then add 1
    let y: i32 = 10;
    let z = x / (y as i64);
    // let z = x / y as i64;  // same
    println!("{}", z);

    // (not like that)
    let x: i64 = (i32::MAX as i64) + 1;  // take the max number for i32 and store it has i64, then add 1
    let y: i32 = 10;
    let z = (x as i32) / y;  // overflow
    println!("{}", z);


    // convert String from to user input to number
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let int_input: i64 = input.trim().parse().unwrap();  // trim = remove the invisible char; parse = return the result, parse the string to the type; unwrap into the type
    println!("{}", int_input + 2);
}


fn tutorial7() {
    // condition and control flow
    let cond = 2 < 3;
    println!("{}", cond);
    let cond = (2 as f32) < 3.3;  // should convert the type
    println!("{}", cond);

    // compound condition (and &&; or ||; not !)

    let food = "cookie";

    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "fruits"{
        println!("That sounds healthy!");
    } else {
        println!("Oh, that's too bad!");
    }

    // similar
    match food {
        "cookie" => println!("Yummy cookie"),
        "bread" => println!("boring"),
        _=> println!("that's too bad")
    }
}

fn tutorial8(){
    // statment (let y = 5; is a statment and doesn't return anything)
    // expression: evaluate something and return something (can be stored in a variable)
    // is an expression:
    // {
    //     let x = 3;
    //     x + 1
    // };
    // here is the error if there is a semicolumn at the line x + 1:
    // error[E0277]: `()` doesn't implement `std::fmt::Display`
    //    --> src\main.rs:231:38
    //     |
    // 229 |         x + 1; // no semicolumn => return what is evaluated
    //     |              - help: remove this semicolon
    // 230 |     };
    // 231 |     println!("x = 3 then x + 1: {}", number);
    //     |                                      ^^^^^^ `()` cannot be formatted with the default formatter
    //     |
    //     = help: the trait `std::fmt::Display` is not implemented for `()`
    //     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

    // For more information about this error, try `rustc --explain E0277`.
    let number = {
        let x = 3;
        x + 1 // no semicolumn => return what is evaluated
    };
    println!("x = 3 then x + 1: {}", number);

    let result = add_numbers(20, 30);
    println!("{}", result);
}

fn add_numbers(x: i32, y:i32) -> i32 {
    let result = x + y;
    if result > 10 {
        return result - 10000
    }
    result // expression at the last statement of the function
    // or:
    // return x + y;  (with or without semicolumn)
    // default return value of a function is: ()
}

