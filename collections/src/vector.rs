fn create_vector() -> Vec<i32>{
    let new_vector: Vec<i32> = Vec::new();
    new_vector
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector_testing() {
    let mut a_vector = create_vector();
    let another_vector = vec![1, 2, 3];

    a_vector.push(6);
    a_vector.push(7);
    a_vector.push(8);
    a_vector.push(9);

    let third = another_vector[2];
    println!("The Third element is : {}", third);

    let third = another_vector.get(3);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("The third element does not exist."),
    }

    for i in &mut a_vector {
        *i += 10;
    }

    for i in a_vector {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(420.69),
        ];
    
    for i in row {
        match i {
            SpreadsheetCell::Int(x) => println!("The Cell contains an int with value {}", x),
            SpreadsheetCell::Text(x) => println!("The Cell contains a string with value {}", x),
            SpreadsheetCell::Float(x) => println!("The Cell contains a float with value {}", x),
        }
    }
}