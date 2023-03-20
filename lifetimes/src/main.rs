fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    let string1 = String::from("Das ist ein TEst");
    let result;
    {
        let string2 = String::from("Das ist ein noch laengerer Test");

        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result)
}
