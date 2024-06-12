struct Point<T, U>{
    x: T,
    y: U,
}

fn generic<T>(x: SGeneric<T>) -> T {
    x
}

fn largest<T>(x:i32)-> &T{
    &x
}

fn main() {
    let work = Point { x: 5, y: 5 };
}
