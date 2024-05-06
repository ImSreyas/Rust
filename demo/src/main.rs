use std::io::{Write, stdout, stdin};

fn main() {
    //user input from console
    let mut a: String = String::new();
    print!("Enter the first value : ");
    stdout().flush().unwrap();
    stdin().read_line(&mut a).expect("Something went  wrong");
    let val1: i32 = a.trim().parse().unwrap();
    let mut b: String = String::new();
    print!("Enter the second value : ");
    stdout().flush().unwrap();
    stdin().read_line(&mut b).expect("Something went  wrong");
    let val2: i32 = b.trim().parse().unwrap();
    print!("sum is : {}", sum(val1, val2));
}
fn sum(a: i32, b: i32) -> i32{
    a + b
}