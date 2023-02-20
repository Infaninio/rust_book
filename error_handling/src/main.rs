use std::fs::File;
use std::io::{self, Read};

fn panic_function() -> i32 {
    let v = vec![1, 2, 3];

    v[99];
    32
}

fn error_handling_with_match(file_name: &str) -> File {
    let greeting_file_result = File::open(file_name);

    match greeting_file_result {
        Ok(file) => file,
        Err(error) =>  match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can not create file {:?}", e)
            },
            _ => panic!("Can't open File: {:?}", error)   
        }
    }
}


fn error_handling_with_unwrap(file_name: &str) -> File {
    // unwrap will get the value and otherwise panic!
    File::open(file_name).unwrap()
}


fn error_handling_with_expect(file_name: &str) -> File {
    File::open(file_name).expect("hello.txt should be included in this project.")
}


fn read_username_from_file() -> Result<String, io::Error> {
    // Propagating the error
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    // Or function chaining
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    
    
    Ok(username)
}

fn read_last_letter_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


fn main() {
    // panic_function();
    // let greeting_file_result = error_handling_with_match("hello.txt");
    // error_handling_with_unwrap("hello.txt");
    error_handling_with_expect("hello.txt");
}
