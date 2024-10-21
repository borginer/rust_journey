use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let mesgs = vec![
            String::from("dark"),
            String::from("light"),
            String::from("big"),
            String::from("small"),
        ];
        for val in mesgs {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let mesgs = vec![
            String::from("holla"),
            String::from("lambda"),
            String::from("poker"),
            String::from("banana"),
        ];
        for val in mesgs {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    
    for received in rx {
        println!("Got message: {received}");
    }
}
