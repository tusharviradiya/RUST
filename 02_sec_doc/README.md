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

## {:?}

- {:?} is used within the println! macro to print **debug representations** of values.
- The debug representation of values in Rust is a textual representation of a value that is suitable for debugging purposes.
- It typically includes enough information to identify the value and its internal structure, but it's not intended for human-readable output in the same way as a formatted string might be.

## Tuples

```rust
let x = (1, "hello");
let x: (i32, &str) = (1, "hello");
let y:(i8, String, bool, char) = (-6, String::from("tushar"), true, 'a');
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

//for in
for x in 0..10 {
    println!("{}", x); // x: i32
}
for element in array.iter() {
    println!("Current element is: {}", element);
}
// array.iter() returns an iterator over the elements of the array, allowing you to iterate over each element using a for loop.
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

- A ‘vector’ is a dynamic or ‘growable’ array, implemented as the standard library type Vec<T>. The T means that we can have vectors of any type

```rust
let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
let v = vec![0; 10]; // A vector of ten zeroes.
//access element
println!("v is {}", v[3]);
println!("v is {:?}", v);

// Accessing elements
let i: usize = 0;
let j: i32 = 0;
// Works:
v[i];
// Doesn’t:
v[j]; // you cannot index with an i32
```

#### Out-of-bounds Access

```rust
let v = vec![1, 2, 3];
println!("Item 7 is {}", v[7]);

//error handling
let v = vec![1, 2, 3];
match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
}
// you can use methods like get or get_mut that return None when given an invalid index
```

## Macro

- macros are a powerful feature that allows you to write code that writes other code.
- There are two main types of macros in Rust:
  1. Declarative Macros (macro_rules!)
  1. Procedural Macros

## Iterating

```rust
// you can iterate through its elements with for. There are three versions:
fn main() {
let mut v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
}
```
- **Note** : You cannot use the vector again once you have iterated by taking ownership of the vector. You can iterate the vector multiple times by taking a reference to the vector whilst iterating.
```rust
let v = vec![1, 2, 3, 4, 5];
for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
```
