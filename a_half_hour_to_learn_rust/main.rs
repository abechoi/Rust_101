use std::cmp::{min, max};
use std::marker::Copy;

struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn introduce(self){
        println!("Hello, my name is {}!", self.name)
    }
}

struct Number {
    odd: bool,
    value: i32,
}

impl Copy for Number { }
impl Clone for Number {
    fn clone(&self) -> Number {
        *self
    }
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
    let x =  Person {name: String::from("Abe"), age: 35};
    println!("Struct Person: x.name = {}, x.age = {}", x.name, x.age);
    x.introduce();

    // Use len() function.
    let message = "How long is this message?";
    println!("The message length is {} characters", message.len());

    // Use min() and max() functions.
    let a = (2, 5);
    let b = (9, 1);
    
    let min = min(a.0, a.1);
    let max = max(b.0, b.1);

    println!("min = {}, max = {}", min, max);

    // Vectors - print out a vector with even numbers only
    let list: Vec<i32> = vec![1,2,3,4,5,6];
    let sublist: Vec<i32> = list.into_iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", sublist);

    // Create a function using match
    let number: i32 = 2;
    println!("{}", number_to_string(number)); 

    // Create a function that matches patterns of a struct
    let x: Number = Number {odd: true, value: 1};
    let y: Number = Number {odd: false, value: 2};

    odds_or_evens(x);
    odds_or_evens(y);


    let z = x;
    odds_or_evens(z);

}

fn number_to_string(number: i32) -> String{
    match number{
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        _ => String::from("Error: number out of bounds...")
    }
}

fn odds_or_evens(n: Number){
    match n {
        Number {odd: true, value} => println!("{} is odds!", value),
        Number {odd: false, value} => println!("{} is evens!", value),
    }
}