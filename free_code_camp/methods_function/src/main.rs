// struct Person {
//     hieght: u32,
//     weight: u32,
// }

// impl Person {
//     fn area(&self) -> u32 {
//         self.hieght * self.weight
//     }
// }

struct Person {
    hieght: u32,
    weight: u32,
} 

impl Person {
    fn new(hieght: u32, weight: u32) -> Self {
        Self { hieght, weight }
    }
}


fn main() {
    println!("Hello, world!");
}
