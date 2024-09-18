use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    open_file();
    let _data_res = read_data_from_file();
}

fn open_file() {
    let file_res = File::open("hello.txt");

    let _file = match file_res {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(create_error) => 
                panic!("Create file failed: {create_error:?}"),
            },
            other_error => {
                panic!("Open failed: {other_error:?}");
            }
        },
    };
}

fn read_data_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    
    Ok(username)
}

fn _read_data_from_file_std() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
