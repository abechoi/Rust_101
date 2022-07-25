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

    // Destructuring Tuples
    let (x , y) = pair;

    println!("{} {}", x , y);


}