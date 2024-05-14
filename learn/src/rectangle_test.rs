#[derive(Debug)]
struct Point {
    x: i16,
    y: i16
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self {
            x, 
            y
        }
    }
}

fn main() {
    let point: Point = Point::new(10, 20);
    let mut b = point.x;
    b = 25;
    println!("The point is a: {} and b: {}", point.x, point.y);
    println!("Value of b: {}", b);
    
}