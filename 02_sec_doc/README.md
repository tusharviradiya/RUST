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

## Ownership

- There are a few distinct concepts, each with its own chapter.

1. ownership.
1. borrowing, and their associated feature ‘references’.
1. lifetimes, an advanced concept of borrowing.

## Move semantics

```rust
let v = vec![1, 2, 3];

let v2 = v;

println!("v[0] is: {}", v[0]);

/* error: use of moved value: `v`
println!("v[0] is: {}", v[0]);
                        ^ */
```

#### .truncate()

- the .truncate() method is used to shorten a mutable String or Vec<T> (vector) to a specified length.

## Copy types

- However, there’s a trait that changes this behavior, and it’s called Copy.
- We haven’t discussed traits yet, but for now, you can think of them as an annotation to a particular type that adds extra behavior.
- All primitive types implement the Copy trait and their ownership is therefore not moved like one would assume, following the ‘ownership rules’.

## References and Borrowing

```rust
#![allow(unused_variables)]
fn main() {
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // Do stuff with `v1` and `v2`.

    // Return the answer.
    42
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let answer = foo(&v1, &v2);

// We can use `v1` and `v2` here!
}
```

#### &mut references

```rust
fn main() {
let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);
}
```

#### The Rules

- Here are the rules for borrowing in Rust:

First, any borrow must last for a scope no greater than that of the owner. Second, you may have one or the other of these two kinds of borrows, but not both at the same time:

1. one or more references (&T) to a resource,
1. exactly one mutable reference (&mut T).

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

## Issues borrowing prevents

```rust
let mut v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
    v.push(34);
}// error: cannot borrow `v` as mutable because it is also borrowed as immutable
```

## Use after free

```rust
let y: &i32;
{
    let x = 5;
    y = &x;
}

println!("{}", y);
/*
error: `x` does not live long enough
    y = &x;
         ^
note: reference must be valid for the block suffix following statement 0 at
2:16...
let y: &i32;
{
    let x = 5;
    y = &x;
}

note: ...but borrowed value is only valid for the block suffix following
statement 0 at 4:18
    let x = 5;
    y = &x;
}
*/
```

## let y: &i32

- In Rust, declaring a variable as &i32 indicates that it's a reference to an i32 value, rather than the i32 value itself. Here's why you might choose to use a reference (&i32) instead of the value directly (i32)

## Lifetimes

- Lending out a reference to a resource that someone else owns can be complicated. For example, imagine this set of operations.

## Structs

```rust
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}
```

#### Tuple structs

```rust
struct Color(i32, i32, i32);
```

## Enums

- An enum in Rust is a type that represents data that is one of several possible variants. Each variant in the enum can optionally have data associated with it.

```rust
fn main() {
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
}
```

## Match

```rust
fn main() {
let x = 5;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
}
```

#### Matching on enums

```rust
fn main() {
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn quit() { /* ... */ }
fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
fn move_cursor(x: i32, y: i32) { /* ... */ }

fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x, y: new_name_for_y } => move_cursor(x, new_name_for_y),
        Message::Write(s) => println!("{}", s),
    };
}
}
```

## Patterns

#### Ignoring bindings

```rust
fn main() {
let some_value: Result<i32, &'static str> = Err("There was an error");
match some_value {
    Ok(value) => println!("got a value: {}", value),
    Err(_) => println!("an error occurred"),
}
}
```

#### ref and ref mut

```rust
fn main() {
let x = 5;

match x {
    ref r => println!("Got a reference to {}", r),
}
}
```

#### Ranges

```rust
let x = 1;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
}
```

## Strings

- Rust has two main types of strings: &str and String
- Let’s talk about &str first. These are called ‘string slices’. A string slice has a fixed size, and cannot be mutated. It is a reference to a sequence of UTF-8 bytes.
- A String is a heap-allocated string. This string is growable, and is also guaranteed to be UTF-8. Strings are commonly created by converting from a string slice using the to_string method.

```rust
let mut s = "Hello".to_string(); // mut s: String
let s:String = String::from("tushar");
```

## Generics

- when writing a function or data type, we may want it to work for multiple types of arguments. In Rust, we can do this with generics

```rust
fn main() {
enum Option<T> {
    Some(T),
    None,
}
}
//Rust’s standard library provides a type, Option<T>, that’s generic
let x: Option<i32> = Some(5);

// Generics don’t have to only be generic over one type. Consider another type from Rust’s standard library that’s similar, Result<T, E>
fn main() {
enum Result<T, E> {
    Ok(T),
    Err(E),
}
}
```

#### Generic functions

```rust
fn takes_anything<T>(x: T) {
    // Do something with `x`.
}
```

#### Generic structs

```rust
fn main() {
struct Point<T> {
    x: T,
    y: T,
}

let int_origin = Point { x: 0, y: 0 };
let float_origin = Point { x: 0.0, y: 0.0 };
}
```

## Traits

- except that we first define a trait with a method signature, then implement the trait for a type.

```rust
fn main() {
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
}
```

## Drop

- The Drop trait provides a way to run some code when a value goes out of scope. For example.

```rust
struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let x = HasDrop;

    // Do stuff.

} // `x` goes out of scope here.

```

## if let

- if let permits patterns matching within the condition of an if statement. This allows us to reduce the overhead of certain kinds of pattern matches and express them in a more convenient way.

```rust
fn main() {
let option = Some(5);
fn foo(x: i32) { }
fn bar() { }
if let Some(x) = option {
    foo(x);
} else {
    bar();
}
}
```

## while let

```rust
fn main() {
let mut v = vec![1, 3, 5, 7, 11];
loop {
    match v.pop() {
        Some(x) =>  println!("{}", x),
        None => break,
    }
}
}
```

## Closures

```rust
fn main() {
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
}
```

- We create a binding, plus_one, and assign it to a closure. The closure’s arguments go between the pipes (|), and the body is an expression, in this case, x + 1. Remember that { } is an expression, so we can have multi-line closures too.

## Crates and Modules

- some of your functionality is private, and some is public. To facilitate these kinds of things, Rust has a module system.

## const and static

#### const

- Constants live for the entire lifetime of a program. More specifically, constants in Rust have no fixed address in memory. This is because they’re effectively inlined to each place that they’re used. References to the same constant are not necessarily guaranteed to refer to the same memory address for this reason.

#### static

- Rust provides a ‘global variable’ sort of facility in static items. They’re similar to constants, but static items aren’t inlined upon use. This means that there is only one instance for each value, and it’s at a fixed location in memory.

## Attributes

```rust
#[test]
#![test]
```

- difference between them

```rust
#[foo]
struct Foo;

mod bar {
    #![bar]
}
```

- The #[foo] attribute applies to the next item, which is the struct declaration. The #![bar] attribute applies to the item enclosing it, which is the mod declaration. Otherwise, they’re the same. Both change the meaning of the item they’re attached to somehow.
- Attributes may also have additional data and key/value

```rust
#[inline(always)]
#[cfg(target_os = "macos")]
```

## Type Aliases

- The type keyword lets you declare an alias of another type.
- You can then use this type as if it were a real type.

```rust
type Name = String;

let x: Name = "Hello".to_string();
```

## Casting Between Types

#### as

```rust
let x: i32 = 5;

let y = x as i64;
```

#### transmute
- The transmute function is very simple, but very scary. It tells Rust to treat a value of one type as though it were another type. It does this regardless of the typechecking system, and completely trusts you.