#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn run() {
    let rect1 = Rectangle::new(15, 25);
    let rect2 = Rectangle::new(10, 34);
    let square = Rectangle::square(26);

    println!("Area of Rect1: {}", rect1.area());
    println!("Rect1 fits into Rect2: {}", rect1.can_hold(&rect2));
    println!("Square fits into Rect1: {}", square.can_hold(&rect1));
}
