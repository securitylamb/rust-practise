/*
    enum and pattern matching
*/
/*we can make our code more concise by making enumkind to store the data by adding ()on them */
#[allow(dead_code)]
#[allow(unused_variables)]

pub fn practise_enum_part_1() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg: Message = Message::Move { x: 1, y: 2 };
    // matching pattern
    if let Message::Move { x: a, y: b } = msg {
        println!("{} {}", a, b);
    }
    // this is the actual destucturing of value
}
pub fn enumerate_in() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    /*
       to use iter() we have to use enumerate
    */
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}
pub fn return_value_from_break() {
    let mut i = 0;
    let result = loop {
        if i == 20 {
            break i;
        };
        i += 1;
    };
    println!("{}", result);
}
pub fn internal_looping() {
    let mut count = 0;
    'outer: loop {
        'inner: loop {
            if count > 20 {
                break 'inner;
            }
            count += 2;
        }
        if count == 30 {
            println!("{}", count);
            break 'outer;
        }
        count += 1;
        // if
    }
}
pub fn pattern_matching() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..'z'| 'A'..'Z'| '0'..'9'));
    }

    println!("Success!");
}
pub fn main() {
    // enum allow us to enumerate upon list of varience
    practise_enum_part_1();
    enumerate_in();
    return_value_from_break();
    internal_looping();
    pattern_matching();
}
