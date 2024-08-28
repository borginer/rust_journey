fn main() {
    basic(4);
    if_assign(true);
    counter();
    array_while();
    array_for();
    countdown();
}

fn basic(x: i32) {
    if x < 5 {
        println!("x < 5");
    } else {
        println!("x >= 5");
    }
} 

fn if_assign(cond: bool) {
    let x = if cond { 3 } else { 9 };

    println!("x is {}", x);
}

fn counter() {
    let mut count = 0;

    let result = loop {
        count += 1;
        
        if count == 10 {
            break count * 3;
        }
    };

    println!("result: {}", result);
}

fn array_while() {
    let a = [10, 30, 70, 65, 72];
    let mut idx = 0;

    while idx < a.len() {
        println!("element {idx} is {}", a[idx]);
        
        idx += 1;
    }

}

fn array_for() {
    let a = [10, 30, 70, 65, 72];

    for element in a {
        println!("element is {element}");
    }
}

fn countdown() {
    for num in (1..4).rev() {
        println!("{num}");
    }
    println!("BOOM");
}
