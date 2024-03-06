# RUST

- topics which is covered in this [documentation](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/variable-bindings.html)

# Topics

## Variable Bindings

- Virtually every non-'Hello World’ Rust program uses variable bindings. They bind some value to a name, so it can be used later. let is used to introduce a binding.

## Patterns

- the left-hand side of a let statement is a ‘pattern’, not a variable name.

## Type annotations

- Rust is a statically typed language, which means that we specify our types up front, and they’re checked at compile time. So why does our first example compile? Well, Rust has this thing called ‘type inference’.
- We can add the type if we want to, though. Types come after a colon (:).

## Mutability

- By default, bindings are immutable. This code will not compile.

## Scope and shadowing

- Variable bindings have a scope - they are constrained to live in the block they were defined in. A block is a collection of statements enclosed by { and }.

## Functions

- Every Rust program has at least one function, the main function

## Expressions vs. Statements

- There are only two kinds of statements, and everything else is an expression.
- Expressions return a value, and statements do not.

1. Early returns

```rust

#![allow(unused_variables)]
fn main() {
fn foo(x: i32) -> i32 {
    return x + 1;
}
}
```

2. Diverging functions

```rust

#![allow(unused_variables)]
fn main() {
fn diverges() -> ! {
    panic!("This function never returns!");
}
}
```

3. Function pointers

```rust

#![allow(unused_variables)]
fn main() {
let f: fn(i32) -> i32;
}
```

## Primitive Types

```rust

let y: bool = false;
let x = 'x';
let y = 1.0; // `y` has type `f64`.
let a = [1, 2, 3]; // a: [i32; 3]
```

1. Variable-size types

- isize : Its size is equal to the size of a pointer on the target architecture.
- usize : Its size is also equal to the size of a pointer on the target architecture.
- They provide a portable way to represent pointer-sized integers without explicitly specifying their size

## Slices

```rust
#![allow(unused_variables)]
fn main() {
let a = [0, 1, 2, 3, 4];
let complete = &a[..]; // A slice containing all of the elements in `a`.
let middle = &a[1..4]; // A slice of `a`: only the elements `1`, `2`, and `3`.
}
```

## Tuples

```rust
let x = (1, "hello");
let x: (i32, &str) = (1, "hello");
```

1. Tuple Indexing

```rust
let tuple = (1, 2, 3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);
```

## Functions

```rust
fn foo(x: i32) -> i32 { x }
let x: fn(i32) -> i32 = foo;
```

- In this case, x is a ‘function pointer’ to a function that takes an i32 and returns an i32.

## Comments

- In Rust, /// and //! are both used for writing documentation comments, but they serve slightly different purposes.

## if

```rust
#![allow(unused_variables)]
fn main() {
let x = 5;

if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    println!("x is not five or six :(");
}
}
```

## Loops

```rust
//loop
loop {
    println!("Loop forever!");
}

//while
while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
}

//for
for (x = 0; x < 10; x++) {
    printf( "%d\n", x );
}

//for in
for x in 0..10 {
    println!("{}", x); // x: i32
}
```

1. Enumerate

```rust
fn main() {
for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
}
}
```

- When you need to keep track of how many times you have already looped, you can use the .enumerate() function.
- end loop early use break and continue.

2. Loop labels

```rust
fn main() {
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
        if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
        println!("x: {}, y: {}", x, y);
    }
}
}
```

## Vectors