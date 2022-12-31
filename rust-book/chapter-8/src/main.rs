use std::collections::{HashMap, hash_map};

#[derive(Debug)]
enum SpreadSheet {
    Cell(u32),
    Row(u32),
    Column(f32)
}
fn main() {
    let vector = vec![SpreadSheet::Cell(5), SpreadSheet::Row(10)];

    let mut string = "Hey there";
    let string = string.to_string();
    println!("Equivalent? {}", String::from("Hey there") == string);

    let mut s = String::from("hey there");
    s.push_str("hey there");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2);

    println!("{}", s2);

    let a1 = String::from("Hello ");
    let a2 = String::from("world");
    let a3 = a1 + &a2;
    println!("{}", a3);
    

    let stringer = String::from("Bye there");
    // let chair = &stringer[0];

    let manual_string = "hey there";
    let man_char = &manual_string[0..3];
    println!("{} ", man_char);

    for char in "This is a string".chars() {
        println!("{char}");
    }

    for char in "This is a string".bytes() {
        println!("{char}");
    }

    let mut hashmapping = HashMap::new();

    hashmapping.insert(String::from("first"), 123);
    hashmapping.insert(String::from("Bye though"), 12);

    println!("{}", hashmapping.get(&String::from("first")).unwrap());


    for (k, v) in &hashmapping {
        println!("{} {}", k, v);
    }

}
