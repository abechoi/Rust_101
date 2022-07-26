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
}
