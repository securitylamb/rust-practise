#![allow(dead_code)]
// #![allow(unused_variables)]
pub fn des_tuple() {
    let (x, y) = (1, "String");
    assert_eq!(x, 1);
    assert_eq!(y, "String");
    println!("destructing tuple");

    // example 1
}
pub fn example_tuple() {
    let (x, _): (i32, i32);
    (x, _) = (3, 4);
    println!("{x}");
    /*
       we can also check the type using
       type_of()
    */
}

pub fn check_type() {
    assert!(1i32 - 2i32 == -1i32);
    println!("{}", 1i32 - 2i32);
    println!("Success")
}
pub fn main() {
    let x: i32 = 0;
    /*
       we can left uninstaliaed variable using
       => _(variable_name)
    */
    let _y: i32;
    assert_eq!(x, 0);
    println!("Success");

    // max and min value across various datatype
    println!("max value of i8 is =>  {}", i8::MAX);
    println!("min value of i8 is => {}", i8::MIN);

    // for loop iterating through some range

    for i in -3..=3 {
        print!("{i} ");
    }
    println!();
    // day2::des_tuple();
    // day2::example_tuple();
    // day2::check_type();
}
