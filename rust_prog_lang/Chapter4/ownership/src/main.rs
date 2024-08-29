fn main() {
    simple_string();
    deep_copy();
    
    let s1 = String::from("you take me ");
    let s1 = take_os_and_give_back(s1);

    println!("{s1}");

    println!("s1 len: {}", calc_len(&s1));

    let s2 = String::from("hello world");
    println!("first word of \"hello world\" is {}", first_word(&s2));

    int_slice_test();
}


fn simple_string() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");
}

fn deep_copy() {
    let s1 = String::from("very");

    let mut s2 = s1.clone();
    s2.push_str(" funny");

    println!("{s1} {s2}");
}

fn take_os_and_give_back(mut s: String) -> String {
    s.push_str("and give me back");
    s
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn int_slice_test() {
    let a = [1, 2, 3, 4, 5];

    let a_slice = &a[..=3];

    assert_eq!(a_slice, [1, 2, 3, 4]);
}
