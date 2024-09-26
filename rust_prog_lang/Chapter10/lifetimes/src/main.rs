struct StringData<'a> {
    _part: &'a str,
}

fn main() {
    let s1 = String::from("lalala funny joke");
    let s2 = "im a very long string example";

    let s3 = longest(&s1, &s2);
    println!("longer string:\n{s3}");


    let a1 = String::from("first sentence. second sentence.");
    let first_sentence = a1.split('.').next().unwrap();
    let _sdata = StringData {
        _part: first_sentence,
    };
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
