pub fn size_of_string() {
    /*
       1. What size_of_val Returns for String:

           * The String type in Rust is a heap-allocated smart pointer. Internally, it consists of:
               * A pointer to the actual data on the heap.
               * The length of the string.
               * The capacity of the allocated memory.
           * On most platforms, the size of the String struct itself is 24 bytes on 64-bit architectures (8 bytes for the pointer, 8 bytes for length, and 8 bytes for capacity).
    */
    use std::mem::size_of_val;
    let str = String::from("cc");
    println!("Size of String struct: {}", std::mem::size_of_val(&str));
    /*
       so the size of str is known at compiler time which is 24 bytes as each field in string is usize which is 3 * 8 bytes = 24 bytes ie for 64 bit system
    */
    println!("this is amount of bytes of data {}", str.len());
}

fn deep_copy() {
    let str1 = String::from("value");
    let str2 = str1.clone();
    println!("{}", str2);
}
fn learning_ownership() {
    let str1 = String::from("value");
    send_ownership_string(str1);

    pub fn send_ownership_string(some_string: String) {
        // the ownership of stirng is passed here ie string using heap allocator so its value is moved not copied
        println!("{}", some_string);
    }
    // imp
    /*
       borrow of moved value: `str1` value borrowed here after move
    */
    // println!("{}", str1);

    let x: u32 = 4;
    send_some_ownership_for_primary_value_or_stack_values(x);

    pub fn send_some_ownership_for_primary_value_or_stack_values(x: u32) {
        // this willnot give error as its values is copied and not moved
        println!("{x}");
    }
    print!("{x}");
}
pub fn convert_string_to_vec() {
    let s1 = String::from("value");
    //this method into_bytes() is used to convert string to vec but it consume the string
    // instead use as_bytes() which take refrence so it does not consume the string
    let x = s1.into_bytes();
    println!("{:#?}", x);
}
pub fn smart_pointer() {
    // we can use the box type to  send primary data or stack type data to heap and the refrence
    // hold the address to the heap location of the data
    let x: Box<i32> = Box::new(2);
    // *x is printing the actual data on that location as x hold the heap location of the data
    println!("{}", *x);
}
pub fn main() {
    // ownership
    // 3 rules of ownership
    /*
       1. Each value in Rust has an owner
       2. There can only be one owner at a time
       3. When the owner goes out of scope, the value will be dropped
    */
    size_of_string();
    let x: u32 = 1;
    let y: u32 = x;
    /*
       this worked bcz of primary data it copied and not moved
    */
    println!("{} and {}", x, y);
    deep_copy();
    learning_ownership();
    convert_string_to_vec();
    smart_pointer();
}
