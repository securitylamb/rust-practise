#![allow(dead_code)]
use std::collections::HashMap;
use std::io;
pub fn naming_convention() {
    // if you are using const
    /*
     * ->the constant name should be in capital
     * ->it cannot be mut
     */
    const PI: f32 = 2.14;
    //const mut PJ: f32 = 2.14; cannot be mut
}

pub fn looping() {
    for _ in 1..=5 {
        print!("hi ");
    }
    println!();
    let numbers = vec![1, 2, 3, 4, 5];
    for i in numbers {
        print!("{} ", i);
    }
    println!();
}
pub fn break_contiue() {
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        if i == 7 {
            break;
        }
        println!("{}", i);
    }
}

pub fn matching() {
    // let name = "string";
    // let name2 = String::from("string from");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    match input.trim().to_lowercase().as_str() {
        "ramp" => print!("this was a ramp"),
        "string" => println!("this was string"),
        _ => println!("this is non sense"),
    }
}

pub fn defining_hashmap() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );
}
#[allow(unused_variables)]
pub fn defining_vector() {
    let vec = vec![1, 2, 3];
    let new_vec: Vec<i32> = Vec::new();
}

pub fn main() {
    day1::matching();
    excercise_day1::main();
}
