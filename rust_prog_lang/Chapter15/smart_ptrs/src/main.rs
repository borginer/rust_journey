enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{*};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after a: {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after b: {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after c: {}", Rc::strong_count(&a));
    }
    println!("count after c out of scope: {}", Rc::strong_count(&a));
}
