// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// const value = match coin {
//     Coin::Penny => 1,
//     Coin::Nickel => 5,
//     Coin::Dime => 10,
//     Coin::Quarter => 25,
// };

enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    // let alphabet = ['a', 'b', 'c', 'A', 'B', 'C', '0', '1', '2'];

    // for an in alphabet {
    //     assert!(matches!(an, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    // }
    // println!("Success!");
    let mut count = 0;

    let v = vec!(MyEnum::Foo, MyEnum::Bar, MyEnum::Foo);
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }
    assert_eq!(count, 2);
    println!("Success!");
}
