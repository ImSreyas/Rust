#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U
}

impl<T, U> Rectangle<T, U> {
    fn mix<V, W: Clone>(self, other: &Rectangle<V, W>) -> Rectangle<T, W> {
        Rectangle {
            width: self.width, 
            height: (other.height).clone()
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 40, 
        height: 30
    };   
    let rect2 = Rectangle {
        width: 10.0,
        height: 20.0
    };
    let mut rect3 = rect1.mix(&rect2);
    println!("{:?}", rect3);
    rect3.height = 15.0;
    println!("{:?}", rect3);
    println!("{:?}", rect2);
}