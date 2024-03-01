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

- println for print something and ln for new line.
- In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!