fn pattern_binding_operator() {
    // matching and capturing
    let x: Option<i32> = Some(35);
    match x {
        Some(n @ 10..=20) => println!("n {} is between value 10 to 20", n),
        Some(n @ 21..=30) => println!("n {} is between value 20 to 30", n),
        Some(n @ i32::MIN..=9) => println!("n {} is less than 10", n),
        Some(n @ 31..=i32::MAX) => println!("n {} is greater than 30", n),
        None => println!("No value provided"),
    }
    // nested pattern
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 0, y: 10 };
    match point {
        Point { x: x @ 1..=10, y } => println!("{} and {}", x, y),
        _ => {
            println!("NO MATCH")
        }
    }
}

pub fn main() {
    println!("day 9");
    pattern_binding_operator();
}
