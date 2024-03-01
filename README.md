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

- 