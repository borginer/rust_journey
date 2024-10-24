fn main() {
    shadowing();
    multi_patterns();
    destructing();
    ignoring();
    match_guards();
}

fn shadowing() {
    let val = Some(5);
    let bal = 10;

    match val {
        Some(15) => println!("got 15"),
        Some(bal) => println!("match bal: {bal}"),
        _ => println!("default, val = {val:?}"),
    }

    println!("in the end\nval = {val:?}, bal = {bal}");
}

fn multi_patterns() {
    let x = 2;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("something else"),
    }

    match x {
        1..=10 => println!("one through ten"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructing() {
    let p = Point { x: 5, y: 12 };

    let Point { x: a, y: b } = p;
    assert_eq!(5, a);
    assert_eq!(12, b);

    let Point { x, y } = p;
    assert_eq!(5, x);
    assert_eq!(12, y);

    let ((temp, val), Point {x, y}) = ((2, 1), Point {x: 8, y: 32});
    println!("temp = {temp}, val = {val}, point = ({x}, {y})");
}

fn ignoring() {
    let numbers = (1, 5, 4, 8, 42);

    match numbers {
        (first, .., last) => {
            println!("first: {first}, last: {last}");
        }
    }
}

fn match_guards() {
    let num = Some(8);

    match num {
        Some(x) if x % 2 == 0 => println!("{x} is even"),
        Some(x) => println!("{x} is odd"),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
