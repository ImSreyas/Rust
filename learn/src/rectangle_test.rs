struct Point {
    x: i32, 
    y: i32
}

impl Point {
    fn new() -> Point {
        Point {
            x: 10,
            y: 20
        }
    }
}

fn main() {
    let rect: Point = Point::new();
    print!("x:{} and y: {}", rect.x, rect.y);
}