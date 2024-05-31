#[derive(Debug)]
struct Point {
    x: i32, 
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn double(&mut self) -> &Point {
        self.x = self.x * 2;
        self.y = self.y * 2;
        self 
    }
}

fn main() {
    let mut rect: Point = Point::new(10, 20);
    println!("The point is : {:?}", rect);
    rect.double();
    println!("The point is : {:?}", rect);
}