#![allow(unused_variables)]
fn reference_example() {
    let str = String::from("value");
    // this will give error as one mutable refrence can only have 1 refrence at a time
    // let r1 = &mut str;
    // let r2 = &mut str;
    // println!("{}{}", r1, r2);
    let r1 = &str;
    let r2 = &str;
    println!("{} & {}", r1, r2);

    // but we can mutable refernce at a time if they are in different scope
    let mut str1 = String::from("value");
    {
        let r11 = &mut str1;
    }
    let r2 = &mut str1;
    /*
     *
     *  println!("{}{}", r11, r2);
     * this will give error as r11 is out of scope
     */
}
pub fn reference_example_1() {
    let mut str = String::from("value");
    let r1 = &str;
    let r2 = &str;
    println!("{} {}", r1, r2);

    // now what we can do is create mutable refrence of str as we will not be using unmutable refrence r1 and r2 below that ie
    let r3 = &mut str;
    println!("{}", r3);
}
pub fn dangling_refrence_example() {
    /*
    this will casuse error as dangle function  return value its local variable goes out scope and its refrence goes out of scope so our varible points to some garbage value
    */
    // let refrence = dangle();
    // fn dangle() -> &String {
    //     let str = String::from("value");
    //     &str
    // }
}

pub fn type_conversion() {
    // Fix the error with at least two solutions
    /*
       we can use into() method to convert the type of right side to the left pointed type like below
        let s: Box<str> = "hello, world".into();
    */
    let s: Box<str> = "hello, world".into();
    greetings(&s);

    fn greetings(s: &str) {
        println!("{}", s)
    }
}
pub fn string_to_string_slice() {
    // Stirng to &str
    /*
       remember in rust you cannot do
       => String + String ie string concat
       you can do
       => String + string_slice
    */
    let str: String = String::from("value");
    let s = str.as_str();
    /*
       also a refrence to a string is also a stting slice
    */
    let str_slice = &str;
}
pub fn iterating_through_string() {
    let str: String = "string".into();
    // to access chars at a particualr index we can do 2 things
    let char_value = &str[0..1];
    println!("{}", char_value);
    // or we can use the method .chars()
    let v = str.chars().nth(0).unwrap();
    println!("{}", v);
}
pub fn main() {
    println!("this is day 5");
    /*
       we can either have n number of unmutable refrence ie normal refrence ie &var or
       only 1 mutable refrence  but we cannot have both ie all used at the same time
    */
    reference_example();
    reference_example_1();
    dangling_refrence_example();
    type_conversion();
    string_to_string_slice();
    iterating_through_string();
}
