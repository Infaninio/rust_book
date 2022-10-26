fn main() {
    a_function(22, 'h');

    // A multiline comment describing the
    // unusual construct below.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let return_of_five = plus_one(5);
    println!("plus_one(5) returns: {return_of_five}")
}


fn a_function(x: i32, unit_label: char) {
    println!("The value of x is {x}{unit_label}");
}

fn plus_one(x: i32) -> i32 {

    // A comment in front of a statement
    x + 1 // Don't use this commment type
}