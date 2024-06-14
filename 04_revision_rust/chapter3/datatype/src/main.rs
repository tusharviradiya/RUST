fn main() {
    // tuple
    let x:(u32, f64, char) = (500, 6.4, 'a');
    println!("{:?}", x.2);

    // array
    let arr:[i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    println!("{}", arr[0]);
}
