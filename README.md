# RUST

- **rustc** : Rustc is the Rust compiler, responsible for translating Rust source code into executable binaries. It performs various tasks including parsing Rust code, checking it for errors, optimizing performance, and generating machine code. Rustc is typically invoked from the command line with rustc [filename.rs]. It's also possible to use rustc programmatically through its library interface.

- **Cargo** : Cargo is Rust's package manager and build system. It simplifies the process of managing dependencies, building projects, and publishing libraries. Cargo also provides features for testing, documentation generation, and more. With Cargo, developers can easily create, share, and manage Rust projects. Common Cargo commands include cargo new to create a new Rust project, cargo build to compile a project, cargo run to compile and execute a project, cargo test to run tests, and cargo publish to publish a crate to crates.io, Rust's official package registry.

# Guessing Game :

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

# Variables and Mutability

- by default, variables are immutable
- cannot assign twice to immutable variable

```rust
//for assign twice we make variable mutable.
let mut x = 5;
x = 10;
```

### const

- you aren’t allowed to use mut with constants.
- type of the value must be annotated.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Shadowing

- Rustaceans say that the first variable is shadowed by the second
- second variable is what the compiler will see when you use the name of the variable
- We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows
- we not need to make variable mutable when we use let type twice for same variable name.

```rust
fn main() {
    let x = 5;

    let x = x + 1;//for reassign x we need to use let when we not write let so this gives us error : cannot assign twice to immutable variable.

    { //we make scope using {}
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

- if we try to use mut for this, as shown here, we’ll get a compile-time error

```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```

- The error says we’re not allowed to mutate a variable’s type

# Data types

- converted a String to a numeric type using parse

```rust
let guess: u32 = "42".parse().expect("Not a number!");

```

- if we not give u32 : error[E0284]: type annotations needed

## Scalar Types :

1. integer
1. float
1. boolean
1. character

## Integer Types

- integer types default to i32

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

| Number literals | Example        |
| --------------- | -------------- |
| Decimal         | 98_222         |
| Hex             | 0xff           |
| Octal           | 0o77           |
| Binary          | 0b1111_0000    |
| Byte            | (u8 only) b'A' |

## Floating-Point Types

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

## Numeric Operations

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

## The Boolean Type

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## The Character Type

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

## The Array Type

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

# Functions

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

## Statements and Expressions

- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value. Let’s look at some examples.

# Control flow
