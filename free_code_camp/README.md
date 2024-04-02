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

### Array
- fixed size array and same data type and continues memory block in stack memory
- slice which use &str type and 7s[1..4] in this 4 is not counted

### Tuple
- in this different type of data is group together and stored in stack.

### Struct
- A type that is composed of other types
- tuple struct

### Enum
- a type that represents data that is one of several possible variants
- option enum

### Control Flow
- control the flow of operation which is execution in a sequence.

### Pattern Matching
- pattern match using matches! macro and if let 

### Methods
- function associated with particular type or struct
- associated function

### Generics
- placeholder for concrete types
- enable to write more reusable and flexible code
- zero cost abstraction
- const generic : represent compile time constant value.
- 