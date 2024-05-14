use std::io::{self, Write};

fn main() {
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: i32 = input1.trim().parse().expect("Please enter a valid number");

    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: i32 = input2.trim().parse().expect("Please enter a valid number");

    let sum = num1 + num2;

    println!("The sum of {} and {} is: {}", num1, num2, sum);
}
