pub fn getting_value_from_array() {
    let arr = [String::from("value"), "this is a string_slice".to_string()];
    /*this is more safer way of getting value from array */
    println!("{}", arr.get(0).unwrap());
    println!("{}", &arr[0]);
}
pub fn diff_in_size_of_and_size_of_slice() {
    /*
        there is difference in size of array and size of slice
    */
    // lets frst check the size of array

    let arr: [char; 2] = ['a', 'b'];
    println!(
        "size of array of char is which 4 byte each in memory {}",
        std::mem::size_of_val(&arr)
    );
    // checking the size of the slice
    /*
       The issue lies in the misunderstanding between the size of a slice and the size of an array. In Rust, a slice is a fat pointer consisting of two pieces of information:

       A pointer to the data.
       The length of the slice.
       This fat pointer takes up two usize values. On most platforms:

       A usize is 8 bytes on a 64-bit system.
       Hence, the size of a slice reference (&[char]) is 16 bytes (8 bytes for the pointer and 8 bytes for the length).

    */
    let slice_arr: &[char] = &arr[..1];
    println!(
        "size of slice of array of char is in memory {}",
        std::mem::size_of_val(&slice_arr)
    );
}
pub fn slicing_in_array() {

    /*

    slicing in string is different as slicing in array
        * in array we are moving though indices
        * in string we are moving on byte

    &s[0..3];
    if it is a stirng it means 3 bytes of data
    */
}
pub fn long_tuple_error() {
    // tuple cant be too long i think max length is 12
    let tup = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("{:?}", tup);
}
// lets start working with tuple

pub fn working_with_struct() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let user1 = User {
        active: true,
        username: "test".to_string(),
        email: "test@gmail.com".to_string(),
        sign_in_count: 1,
    };
    println!("{:#?}", user1);
    /*
       copying from struct but a smart way
    */
    let user2 = User {
        username: "test_test".to_string(),
        email: "test_test@gmail.com".to_string(),
        ..user1
    };
    println!("{:#?}", user2);

    /*
       there is also new typle od struct called tuple struct
    */
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Tupstruct(i32, i32, i32);
    let cc = Tupstruct(1, 2, 3);
    println!("{:#?}", cc);

    /*
     *  remember when destructing in struct to add struct name to it
     */
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Destructexample {
        x: i32,
        y: i32,
    }
    let x = Destructexample { x: 1, y: 2 };
    // as i want one of the values of the struct
    let Destructexample { x, y } = x;
    println!("{} and {}", x, y);
}
pub fn basic_enum_example() {
    // creating basic enum
    #[derive(Debug)]
    #[allow(dead_code)]
    enum BasicEnum {
        One = 12,
        Two,
        Three,
    }
    // using enum
    /*
        Remember
    // Print the variant name
    println!("{:?}", BasicEnum::One); // Prints: One

    // Print the numeric value (discriminator)
    println!("{}", BasicEnum::One as i32); // Prints: 12

     */
    /*
        for some reason i can only access the i32 type not the &str
     */
    println!("{:?}", BasicEnum::One as i32);
}
pub fn main() {
    getting_value_from_array();
    diff_in_size_of_and_size_of_slice();
    /*do not pass the lenght of slice on the type is required ->  let slice: &[i32]; and not  let slice: &[i32;3] */
    slicing_in_array();
    long_tuple_error();
    working_with_struct();
    basic_enum_example();
}
