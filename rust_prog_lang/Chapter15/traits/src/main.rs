use std::ops::Deref;

struct MyBox<T>(T, i32);

impl <T> MyBox<T> {
    fn new(x: T, id: i32) -> MyBox<T> {
        MyBox(x, id)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("droping mybox with id: {}", self.1);
    }
}

fn main() {
    refs();
    my_box();
    deref_coercion();
}

fn refs() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(x, *y);
    assert_eq!(x, *z);
}

fn my_box() {
    let x = 5;
    let y = MyBox::new(x, 1);

    assert_eq!(x, *y);
}

fn deref_coercion() {
    let m = MyBox::new(String::from("tolkin"), 2);
    print_string(&m);
    print_string(&(*m)[..]);
}

fn print_string(s: &str) {
    println!("s: {s}");
}
