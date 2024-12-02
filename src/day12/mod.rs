pub fn ownership_primitive_nonprimitive() {
    // for primitive data the data is copied so we can do this
    let x = 12;
    let y = x;
    println!("x is {} and y is {} ", x, y);

    let mut str1 = String::from("value");
    // so when we pass the refrence here what will happen
    let str2 = &mut str1;
    str2.push_str("_d_");
    println!(" str 2 is {} ", str2);
    println!(" str 1 is {} ", str1);

    // if i want a copy i can use .copy()
}

pub fn ownership_in_function_for_stack(mut mutable_value_in_function: i32) {
    mutable_value_in_function = 55;
    println!("value is {}", mutable_value_in_function)
}

pub fn ownership_in_function_for_heap(mutable_value_in_function: &String) {
    println!("{}", mutable_value_in_function)
}
pub fn ownership_in_function_for_heap_2(mutable_value_in_function: &mut String) {
    mutable_value_in_function.push_str("string");
    println!("{}", mutable_value_in_function)
}
pub fn derefrence() {
    let mut heap_data: Vec<i32> = vec![5, 6, 7];
    let ref_1: &mut Vec<i32> = &mut heap_data;
    let deref_copy: Vec<i32> = ref_1.clone();
    let move_out: &mut Vec<i32> = ref_1;
    let move_out_again: &mut Vec<i32> = ref_1;
}
pub fn main() {
    ownership_primitive_nonprimitive();
    let stack_variable = 2;
    ownership_in_function_for_stack(stack_variable);
    let mut heap_variable = String::from("ssss");
    /*
       so when a heap variable is passed to function when the function goes out of scope the value also goes out of scope
    */
    ownership_in_function_for_heap(&heap_variable);
    println!("{}", heap_variable);
    // same above example but with mutable reference
    ownership_in_function_for_heap_2(&mut heap_variable);
    println!("{}", heap_variable);

    derefrence();
} 
