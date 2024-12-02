pub fn representing_value_different_format() {
    let x = 255;
    println!(
        "represnting this x in octal {:o} and hexadecimal {:x} and in binary {:b}",
        x, x, x
    )
}
pub fn as_format_in_data() {
    let x: i32 = 10;
    let y: f32 = 12.5;
    let result: f32 = x as f32 + y;
    println!("{}", result)
}

pub fn scope_of_value() {
    let x: i32 = 255;
    let x: char = 'a';
    println!("{}", x);
    {
        let x: i32 = 1234;
        println!("inside a scoope the value of x is different => {}", x)
    }
    println!(
        "outside scope the value of x remain the same not changed due to inside scope of x => {}",
        x
    );

    /*
       what if i dont shadow the value
    */
    let mut y: i32 = 23;
    {
        /*
           the value of y can be accessed and updated  here bcz it is in outerscope
        */
        y = 255;
        println!("{}", y)
    }
    println!("{}", y);

    /*
       the inner scope cannt be accessed outside
    */
    {
        let _z = 55;
    }
    /*
       here value of z cannt be accessed bcz it is inner scope and below println will give error
    */
    //println!("{}", z) // this is giving error
}
pub fn string_tutorial() {
    let mut str = String::from("value_");
    // for pushing character
    str.push('d');
    // for pushing string
    str.push_str("_string");
    println!("{}", str);

    /* most common type of function used with string */
    println!("is string empty => {} , string length is => {} , string capacity is => {} , does string contain this string => {}",str.is_empty(),str.len(),str.capacity(),str.contains("_d_"));

    // we can convert interger to sttring
    let x: i32 = 12;
    let str_1 = x.to_string();
    println!("{}", str_1);
    let str_2 = "this is string slice and converted to String".to_string();
    println!("{}", str_2);
    let str_3 = String::new();
    println!("{}", str_3.len())
}
pub fn array_just_practise() {
    let arr = [0; 10];
    /*
       use this to get the value of array it return the actual value or None
    */
    println!("{:?}", arr.get(11));
}
pub fn vector_tutorial() {
    let mut vec: Vec<i32> = vec![1; 10];
    println!("{:?}", vec);
    //slices in vec
    let slice_vec = &vec[0..=5];
    println!("{:?}", slice_vec);
    vec.push(0);
    println!("{:?}", vec);
    // .contains needs refrence thats why we have to use &0 here
    println!("{:?}", vec.contains(&0));
}
pub fn main() {
    representing_value_different_format();
    as_format_in_data();
    scope_of_value();
    string_tutorial();
    array_just_practise();
    vector_tutorial();
}
