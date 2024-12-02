#![allow(dead_code)]

pub fn data_type_yesturday_lecture_day2() {
    /*
       ineger overflow
       when let say u8 which can hold values from 0 to 255
       and we try to store 256 value in it 2 things can happen
       -> in debug mode this will go to panic mode (panic handle)
       -> in case of release mode it will go -  wrap around” to the minimum of the values the type can hold
       so 256 become 0 and 257 whould be 1

    */
    let max = i32::MAX; // 2,147,483,647
    let wrapped = max.wrapping_add(1); // Wraps around to `i32::MIN`
    println!("Wrapping add: {}", wrapped); // Output: -2,147,483,648

    let min = i32::MIN;
    let wrapped = min.wrapping_sub(1); // Wraps around to `i32::MAX`
    println!("Wrapping sub: {}", wrapped); // Output: 2,147,483,647
}
pub fn print_char(c: char) {
    println!("printing the char {}", c);
}
pub fn empty_function() {
    // example of tuple returning nothing
}
pub fn practise_slice() {
    let (x, y);
    // this is a tuple
    (x, ..) = (3, 4);
    // this is an array
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);
    println!("Success!");
}

pub fn tuple_tut(x: (i32, i32, i32)) {
    /*
       also i think it is nessary to pass size or number of variable the tuple have
    */
    let x_0 = x.0;

    let x_1 = x.1;

    let x_2 = x.2;

    println!("{} & {} & {} ", x_0, x_1, x_2);
    /*
            for tuple data is stored in stack
            for example
            1. Tuple with Stack-Allocated Data
            fn main() {
            let stack_tuple = (42, 3.14, true); // i32, f64, and bool are stack-allocated
            println!("{:?}", stack_tuple);
            2. Tuple with Heap-Allocated Data
            fn main() {
            let heap_tuple = (String::from("hello"), vec![1, 2, 3]);
            println!("{:?}", heap_tuple);
            the tuple is stored instack where as its heap allocated data such as string and vector are stored in heap
    }


    }

         */
}

pub fn basic_array_example() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for i in arr {
        print!("{ }", i);
    }
    println!();
}

pub fn checking_siz_of_value() {
    // Modify `4` in assert to make it work
    use std::mem::size_of_val;

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!",);
}
pub fn divergent_functon() -> ! {
    /*
       function that should never return to the caller
       like in panic macro
    */
    panic!();
}

pub fn main() {
    /*
       important !!!
    */
    // data_type_yesturday_lecture_day2();

    println!("this is day3");
    /*
       unit in rust is tuple returning empty type ie ()
       this can be function returning nothing or empty function
    */
    print_char('k');
    let k = empty_function();
    /*
       this k will return unit or () ie tuple which is 1 byte
    */
    println!("{:?}", k);
    practise_slice();
    tuple_tut((1, 2, 3));
    basic_array_example();
    let v: u16 = 38_u8 as u16;
    println!("{v}");
    println!("{}", 1i32 - 2);
    checking_siz_of_value();
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}
