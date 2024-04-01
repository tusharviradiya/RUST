fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // s.push('!');
    // println!("{}", s);
    // println!("{}", s.to_string());

    let str = String::from("hello");
    let h = &str[..3];
    println!("{}", h);
}

