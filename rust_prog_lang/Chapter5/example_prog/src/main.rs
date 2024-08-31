#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height &&
        self.width >= other.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };

    dbg!(rect1.width); 
    println!("rect1 is {rect1:?}");
    println!("Area of rectangle is {}", rect1.area());
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 60,
        height: 60,
    };

    println!("can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
