use crate::garden::vegetables::Plant;


pub mod garden;

fn main() {
    let plant = Plant {
        name: String::from("potato"),
    };

    println!("plant name: {}", plant.name);
}
