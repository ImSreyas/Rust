struct Data {
    val: i32
}
fn main() {
    // let mut a = vec![1, 2, 3, 4, 5];
    // let b = &mut a[2];
    // *b = 20;
    // println!("the value is : {}", b);
    // for i in a.iter() {
    //     println!("{i}");
    // }
    let a = Data { val: 20};
    something(&a);
    println!("i'm main function and the value of a is : {}", a.val);
}

fn something(a: &Data) {
    println!("i'm something function and the value of a is : {}", a.val);
}