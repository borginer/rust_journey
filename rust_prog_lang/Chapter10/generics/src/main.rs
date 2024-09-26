fn main() {
   let num_list = vec![10, 54, 97, 82, 37];
    println!("largest number: {}", largest(&num_list));

    let p1 = Point { x: 5, y: 50.2 };
    let p2 = Point { x: "hello", y: 'w' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}\np3.y = {}", p3.x, p3.y);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
