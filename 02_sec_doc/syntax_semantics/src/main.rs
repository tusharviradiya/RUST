
#![allow(unused_variables)]
fn main() {
let mut x = 5;
{
    let y = &mut x;
    *y += 1;

    let z = &mut x;
    *z += 4;
}
println!("{}", x);
}