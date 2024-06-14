// start 
#[derive(Debug)]
enum Ip {
    v4{
        x: u8,
        y: u8
    },
    v6(String)
}
fn main() {
    let a: String = "Hello".to_string();
    let val = Ip::v4 {x: 12, y: 20};
    println!("value is : {:?}", val);
}