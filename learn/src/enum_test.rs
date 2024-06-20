#[derive(Debug)]
enum Ip {
    V6(String)
}
fn main() {
    let val = Ip::V6(String::from("hello"));
    println!("value is : {:?}", val);
}
