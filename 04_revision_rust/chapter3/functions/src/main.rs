fn hello(){
    println!("hello world");
}

fn sum(x: i32){
    println!("{}", x);
}

fn expression() -> i32 {
    let x = {
        let y = 10;
        y
    };
    x
}

fn main() {
    hello();
    sum(30);

    let x = {
        let y = 10;
        y
    };
    println!("{}", x);

    println!("{}", expression());

    hello1();
}

fn hello1(){
    println!("hello world");
}