struct Point1 {
    x: i32,
    y: i32,
    z: Stirng,
    q: bool,
    r: f64
}
struct Point2(i32, i32, String, bool, f64);

fn main() {
    // println!("Hello, world!");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    let tup1 = (1, 2, "tushar", true, 5.01);
    println!("{:?}", tup1);

}
