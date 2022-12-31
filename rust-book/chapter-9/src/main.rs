
use std::{fs::File, io::Read}; 
fn main() {
    // panic!("NO!");
    let mut open_string = String::from("");
    let mut file_result = File::open("exists.txt").unwrap();
    file_result.read_to_string(&mut open_string).expect("File needs to exist");
    println!("{open_string}");
    println!("The length of the string is {}", count_string_length(&open_string));

}


fn count_string_length(string: &String) -> usize {
    string.len()
}