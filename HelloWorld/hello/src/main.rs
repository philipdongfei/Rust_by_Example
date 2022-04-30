use std::fmt;
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}



// This is a comment, and is ignored by the compiler
//
fn main() {
    // Statements here are executed when the compiled binary is called
    
    // Print text to the console
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    //
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // 1.2 Formatted print
    println!("{} days", 31);
    println!("{0}, this is {1}.{1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    println!("{} of {:b} people know binary, the other half doesn't", 1, 3);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");


    println!("This struct `{:?}` won't print...", Structure(3));

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:} name.",
        "Slater",
        "Christian",
        actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));


    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:#?}", peter);



}
