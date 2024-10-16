fn main() {
    add_one_closure(); 
    closure_borrow();
    thread_closure();
    count_closure_uses();
}

fn add_one_closure() {
    let add_one = |x| { x + 1 };
    let a = 5;
    let y = add_one(a);

    println!("a: {a}, y: {y}");
}

fn closure_borrow() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");

    let mut borrows_mutably = || { list.push(5); };
    borrows_mutably();

    println!("After calling closure {list:?}");
}

fn thread_closure() {
    use std::thread;
    
    let list = vec![2, 4, 6];
    println!("before moving list: {list:?}");

    thread::spawn(move || println!("from thread {list:?}"))
        .join()
        .unwrap();
}

fn count_closure_uses() {
    let mut list = vec![5, 4, 6, 24];
    println!("list before sorting: {list:?}");
    let mut closure_calls = 0;

    list.sort_by_key(|r| {
        closure_calls += 1;
        r.clone()
    });

    println!("list after sorting: {list:?}
             \nclosure used {closure_calls} times");
}

