enum MyData {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v2 = vec![1, 2, 5];
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);

    let elem: Option<&i32> = v.get(1);
    match elem {
        Some(elem) => println!("2nd elem: {elem}"),
        None => println!("out of bounds"),
    }

    let elem: Option<&i32> = v.get(2);
    match elem {
        Some(elem) => println!("3rd elem: {elem}"),
        None => println!("out of bounds"),
    }

    let first: &i32 = &v[0];
    println!("1st elem: {first}");

    v.push(4);

    for elem in &mut v {
        *elem += 10;
    }

    for elem in &v {
        println!("{elem}");
    }

    let data_vec = vec![
        MyData::Int(3),
        MyData::Float(3.141),
        MyData::Text(String::from("hello world")),
    ];

    print_data(&data_vec[0]);
    print_data(&data_vec[1]);
    print_data(&data_vec[2]);
}

fn print_data(data: &MyData) {
    match data {
        MyData::Int(i) => println!("int: {i}"),
        MyData::Float(f) => println!("float: {f}"),
        MyData::Text(s) => println!("string: {s}"),
    }
}

