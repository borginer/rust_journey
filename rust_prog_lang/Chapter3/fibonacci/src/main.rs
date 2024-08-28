use std::{io, usize};

fn main() {    
    let mut input: String;
    
    loop {
        input = String::new();

        println!("enter n");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number");
                continue;
            },
        };

        println!("number {n} in fibonacci series is {}", calc_fibo(n));
    }
}

fn calc_fibo (n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut arr = [0, 1, 0];
    
    for i in 0..=n-1 {
        arr[(i + 2) % 3] = arr[i % 3] + arr[(i + 1) % 3]; 
    }

    return arr[(n + 1) % 3];
}
