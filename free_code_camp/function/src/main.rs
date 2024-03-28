fn main() {
    println!("{}", sum(2, 3));
    println!("{} + {} = {}", 2, 3, sum(2, 3));
}

fn sum (x: i8, y: i8) -> i8{
    x+y
}
