#[warn(dead_code)]
#[allow(unused_variables)]
pub fn example_one() {
    enum TypesOfCoins {
        Penny,
        Nickle,
        Dime,
        Quater,
    }
    fn value_in_cents(c: TypesOfCoins) -> i32 {
        match c {
            TypesOfCoins::Dime => 10,
            TypesOfCoins::Nickle => 5,
            TypesOfCoins::Penny => 1,
            TypesOfCoins::Quater => 25,
        }
    }
    println!("{}", value_in_cents(TypesOfCoins::Penny));
    println!("{}", value_in_cents(TypesOfCoins::Dime));
    println!("{}", value_in_cents(TypesOfCoins::Nickle));
    println!("{}", value_in_cents(TypesOfCoins::Quater));
}
pub fn destructuring_in_rust() {
    /*
        tuple
    */
    let tup = (1, "2", 3);
    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);
    // getting specific values from tuple
    let (_, k, _) = tup;
    println!("{}", k);
    /*
       struct
    */
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 10, y: 20 };
    let Point { x, y } = point;
    // we can also use different names
    let Point { x: a, y: b } = point;
    println!("{} {}", x, y);
    // printing values of struct with different names
    println!("{} {}", a, b);

    /*
       tuple like enum
    */
    enum Message {
        ChangeColor(u8, u8, u8),
    }
    let msg = Message::ChangeColor(255, 0, 0);

    match msg {
        Message::ChangeColor(r, g, b) => println!("r=>{},g=>{},b=>{}", r, g, b),
        // _ => println!("other msg"),
    }
}
pub fn pattern_matching(n: i32) {
    // we can both check the value and bind the value that we are passing to the match with this
    match n {
        n @ 1..=5 => println!("value is between 1 to 5 and value is : {}", n),
        n @ 6..=10 => println!("value is between 6 to 10 and value is : {}", n),
        n @ _ => println!("value is larger than 10 and value is : {}", n),
    }
}
pub fn main() {
    example_one();
    println!(" ");
    destructuring_in_rust();
    pattern_matching(1);
}
