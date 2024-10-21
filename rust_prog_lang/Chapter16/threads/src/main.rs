use std::thread;
use std::time::Duration;

fn main() {
    basic_thread();
    move_closure();
}

fn basic_thread() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("spawned thread says hello number {i}");
            thread::sleep(Duration::from_millis(250));
        }
    }); 

    for i in 1..5 {
        println!("main thread says hello number {i}");
        thread::sleep(Duration::from_millis(350));
    }

    handle.join().unwrap();
}

fn move_closure() {
    let v = vec![1, 3, 5];

    let handle = thread::spawn(move || {
        println!("moved vector:\n{v:?}");
    });

    handle.join().unwrap();
}
