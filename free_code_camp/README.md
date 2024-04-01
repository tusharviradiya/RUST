# learning of RUST Monster....

### What is macro ??
- a way of defining reusable chunks of code
- ex. like println!, assert_eq! etc.    

### assert_eq!() & assert!

    - for check x and y is equal or not
    - assert! is a cousin of assert_eq!

- A reference is created by & and dereferenced by \*.
- You cannot print out an array in the usual way with {} but you can do a debug print with {:?}.

- in rust language when we not assign any type do i32 and f64 id assigning by default.

### ownership
- set of rule whit rule memory management.
- rules are enforce at compile time.

### Borrowing
- is without taking ownership of data
- in this you are taking reference of data not data itself

### String vs &str
- in rust language memory is allocated by default in stack
- String is mutable and &str is immutable by default
- in String data stored in heap
- in String type variable is hold pointer of data not data
- in string we use .push_str() and .push() both for push string