fn main() {
    #[derive(Debug)]
    enum Gender{
        male(u8),
        female(u8),
        others(u8),
    }
    let one = Gender::male(10);
    println!("{:?}", one);
}
