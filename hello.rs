// fn main() {
    // for i in 0..15{
    //     if i%2==0{
    //         println!("even_{}", i);
    //     }
    //     else{
    //         println!("odd_{}", i);
    //     }
    // }

    // let mut sum = 0;
    // for i in 0..5{
    //     sum += i;
    // }
    // println!("sum : {}", sum);

    // let mut sum = 0.0;
    // for i in 0..5{
    //     sum += i as f32;
    // }
    // println!("sum : {}", sum);

    // let x = sqr(2);
    // println!("{}", x);
// }
// fn sqr(x:i32)-> i32{
//     x * x
// }

// fn modifies(x: &mut f64) {
//     *x = 1.0;
// }

// fn main() {
//     let mut res = 0.0;
//     modifies(&mut res);
//     println!("res is {}", res);
// }

// fn main(){
//     let arr = [1, 2, 3, 4, 5, 6, 7, 8];
//     let slice1 = &arr[..4];
//     let slice2 = &arr[4..];
//     let slice3 = &arr[4..6];
//     println!("{:?}", slice1);
//     println!("{:?}", slice2);
//     println!("{:?}", slice3);
// }

// fn main() {
//     let ints = [1, 2, 3, 4, 5];
//     let slice = &ints;
//     let first = slice.get(0);
//     let last = slice.get(5);

//     println!("first {:?}", first);
//     println!("last {:?}", last);
// }