pub fn associated_function_example() {
    #[derive(Debug)]
    struct Point {
        point_x: u8,
        point_y: u8,
    }
    impl Point {
        fn new() -> Self {
            Self {
                point_x: 1,
                point_y: 2,
            }
        }
    }

    let x = Point::new();
    println!("{:#?}", x);
}
pub fn main() {
    println!("day 10");
    /*
       first making istance calling
    */
    associated_function_example();
}
