use std::io;
use rand::Rng;
use std::io::{Write};
use std::cmp::Ordering;

fn main(){
    // guess();
    // looping();
    pos();
}

fn pos() {
    loop {
        print!("Enter a number : ");
        io::stdout().flush().unwrap();
        let mut val: String = String::new();
        io::stdin().read_line(&mut val).unwrap();
        let val: i32 = match val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };
        if val < 0 {
            println!("The value is negative.");
            continue;
        } 
        println!("value is : {}", val);
    }
}

fn looping() {
    let a = [10, 20, 30, 40];
    let mut sum = 0;
    for (index, value) in a.iter().enumerate() {
        println!("value at {index} is : {value}");
        sum += value;
    }
    println!("sum of the elements in the array is {:?} : {}", a, sum);
}

fn guess() {
    let random:i32 = rand::thread_rng().gen_range(1..101);
    println!("Random values is : {}", random);
    loop {
        print!("Enter a guess : ");
        io::stdout().flush().unwrap();
        let mut a = String::new();

        io::stdin().read_line(&mut a).unwrap();
        let a: i32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) =>  {
                println!("Enter a valid input");
                continue
            }
        };
        match a.cmp(&random) {
            Ordering::Less => println!("value is less."),
            Ordering::Equal =>  {
                println!("equal");
                break;
            },
            Ordering::Greater => println!("value is greater")
        }
    }
}