fn main() {
    simple_iter(); 
    more_iter();
}

fn simple_iter() {
    let v1 = vec![1, 2, 3];

    for val in v1.iter() {
        println!("Elem: {val}");
    }

    let elem_sum : u32 = v1.iter().sum();
    println!("Sum of elements: {elem_sum}");
}

fn more_iter() {
    let v1 = vec![4, 8, 16];

    println!("before iter");
    for val in v1.iter() {
        print!("Elem: {val} ");
    }

    let v2: Vec<i32> = v1.iter().map(|x| { x / 2 }).collect();
    
    println!("\nafter iter");
    for val in v2.iter() {
        print!("Elem: {val} ");
    }
    println!("");
}

#[test]
fn iter_test() {
    let v1 = vec![2, 4, 8];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&8));
    assert_eq!(v1_iter.next(), None);

}
