fn main(){
    let s = String::from("hello");
    let x = s;
    println!("{x}");

    let s1 = String::from("hello");
    let s2 = reference(&s1);
    println!("length of {s1} is {s2}");
    let s3 = &s1;
    let s4 = &s1;
    println!("length of {s3} is {s4}");
}

fn reference(s: &String) -> usize {
    s.len()
}