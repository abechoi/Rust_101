use std::cmp::{min, max};

struct Person {
    name: String,
    age: i32,
}

fn main() {

    // x of type i32 equals 42.
    let x: i32 = 42;
    println!("x = {}", x);

    let x = x + 3;
    println!("x = x + 3 = {}", x);

    // use "_" for unused variables
    let _y: i32 = 43;

    // Tuples:
    // pair.0 of type i32 equals 1
    // pair.1 of type char equals a
    let pair: (i32, char) = (1, 'a');

    println!("pair = ({}, {})", pair.0, pair.1);

    // destructuring tuples
    // x equals pair.0 and y equals pair.1
    let (x , y) = pair;

    println!("(x, y) = ({}, {})", x , y);

    // A pair of brackets declare a block, which has its own scope
    let x = "out";
    {
        let x = "in";
        println!("inner x = {}", x);
    }
    println!("outter x = {}", x);

    // Struct
    let x =  Person {name: "Abe".to_string(), age: 35};
    println!("Struct Person: x.name = {}, x.age = {}", x.name, x.age);

    // Create and use a function with len() function.
    let message = "How long is this message?";
    println!("The message length is {} characters", message.len());

    // Create and use a function with min() and max() function.
    let a = (2, 5);
    let b = (9, 1);
    
    let min = min(a.0, a.1);
    let max = max(b.0, b.1);

    println!("min = {}, max = {}", min, max);

}