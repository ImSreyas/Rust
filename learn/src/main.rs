use std::io;
use rand::Rng;
use std::io::{Write};
use std::cmp::Ordering;

fn main(){
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