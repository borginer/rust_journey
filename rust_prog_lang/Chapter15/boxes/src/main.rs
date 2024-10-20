enum List {
    Cons(i32, Box::<List>),
    Nil,
}

use List::{*};

fn main() {
    basics();
    cons_list();
}

fn basics() {
    let b = Box::new(5);
    println!("b = {b}");

    let c = *b + 5;
    println!("c = {c}");
}

fn cons_list() { // lol
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let (mut a, mut b, mut c) = (0, 0, 0);
    if let List::Cons(val, next) = list {
        a = val;
        if let List::Cons(val, next) = *next {
            b = val;
            if let List::Cons(val, _) = *next {
                c = val;
            }
        }
    }
    println!("first item: {a}\nsecond item: {b}\nthird item: {c}");
}


