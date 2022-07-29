<div align="center">
<h1>Rust Tutorial</h1>
<h2>Introduction to Rust Programming</h2>
<p>By Tech With Tim</p>
</div>

<p align="center">
Youtube: Rust Tutorial from [Tech With Tim](https://www.youtube.com/watch?v=T_KrYLW4jw8&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ&ab_channel=TechWithTim)</p>

1.  [Installation](#Installation)
2.  [Rust Tools](#Rust-Tools)
3.  [Variabes Constants and Shadowing](#Variabes-Constants-and-Shadowing)
4.  [Data Types](#Data-Types)
5.  [Console Input](#Console-Input)
6.  [Chapter VI](#chapter-vi)
7.  [Chapter VII](#chapter-vii)
8.  [Chapter VIII](#chapter-viii)
9.  [Chapter IX](#chapter-ix)
10. [Chapter X](#chapter-x)


## Installation

For MacOS, run the command and restart terminal.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Rust Tools

Create and run a new Rust project with Cargo package manager (Recommended).
```
# creates new project
cargo new project_name

# compiles and runs project
cargo run

# updates dependencies and libraries
cargo build
```

```
# builds a binary file from rs file
rustc main.rs
```

```
# format to optimize rs file
rustfmt main.rs
```

## Variabes Constants and Shadowing

Variables
```
# create a constant i32
let x: i32 = 1;

# create a mutable i32
let mut x: i32 = 2;
```

Scopes (Shadowing)
```
fn main(){
    let x = 0;
    { // scope starts
        let x = x + 1; // added 1 to the outer x
        println!("x = {}", x); // 1
    } // scope ends
    let x = x + 2; // does not add the inner x
}
```

Constants
```
const SECONDS_IN_MINUTE: u32 = 60;
```

## Data Types

Integers: i8 i16 i32 i64 i128

Unsigned Integers: u8 u16 u32 u64 u128 u256

Floats: f32 f64

Booleans: bool - 1 or true, 0 or false

Character: char - any character in single quotation marks, 'a'

Tuples
```
# creates a tuple
let tup: (i32, bool, char) = (1, true, 'a');

# print each element
println!("({}, {}, {})", tup.0, tup.1, tup.2);
```

Arrays
```
# creates an array of i32 with 5 elements
let arr: [i32; 5] = [1, 2, 3, 4, 5];
```

## Console Input

IO module is used to read user input via command line.
```
# call the io (input/output) module
use std::io;

fn main() {

    let mut input = String::new();

    # reads line from command line
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}

```

## Chapter VI

Enter information about Chapter VI.

```
# this ia a code block for chapter vi.
```

## Chapter VII

Enter information about Chapter VII.

```
# this ia a code block for chapter vii.
```
## Chapter VIII

Enter information about Chapter VIII.

```
# this ia a code block for chapter viii.
```

## Chapter IX

Enter information about Chapter IX.

```
# this ia a code block for chapter ix.
```

## Chapter X

Enter information about Chapter X.

```
# this ia a code block for chapter x.
```