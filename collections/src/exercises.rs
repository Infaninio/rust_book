use std::{collections::HashMap};

fn find_median(input_vec: &mut Vec<i32>) -> i32 {
    input_vec.sort();
    let middle_element = input_vec.len() / 2;

    if middle_element * 2 == input_vec.len() && input_vec.len() > 0 {
        return (input_vec[middle_element] + input_vec[middle_element - 1]) / 2;
    }
    
    match input_vec.get(middle_element) {
        Some(x) => *x,
        None => 0, 
    }
}

fn find_mode(input_vec: &Vec<i32>) -> i32 {
    let mut counter_map: HashMap<i32, i32> = HashMap::new();

    for value in input_vec {
        let count = counter_map.entry(*value).or_insert(0);
        *count += 1;
    }
    let mut max_val = 0;
    let mut max_index = 0;
    for (key, value) in counter_map {
        if value > max_val {
            max_index = key;
            max_val = value;
        }
    }

    max_index
}

pub fn ex_1 () {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle
    // position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut a_vector = vec![32, 364, 15, 15, 745, 1, 56, 167, 1563, 11];
    let mut empty_vector: Vec<i32> = Vec::new();
    println!("The Median of the vector is: {}", find_median(&mut a_vector));
    println!("Median of an empty vector: is {}", find_median(&mut empty_vector));

    println!("The mode of a vector is : {}", find_mode(&a_vector))

}

pub fn ex_2 () {
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and 
    // “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to 
    // the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
}

pub fn ex_3 () {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a 
    // department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let 
    // the user retrieve a list of all people in a department or all people in the company by department, 
    // sorted alphabetically.
}