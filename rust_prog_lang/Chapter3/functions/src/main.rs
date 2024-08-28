fn main() {
    println!("Hello, world!");

    another_function();

    num_function(6);

    expresion_function();

    println!("five function return: {}", five());

    println!("five then plus_one: {}", plus_one(five()));
}

fn another_function() {
    println!("Another function.");
}

fn num_function(num: i32) {
    println!("the num is {num}");
}

fn expresion_function() {
    let y = {
        let x = 3;
        x * 2
    };

    println!("the value of y is {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
