use std::vec;

fn create_stack(max_size: usize) -> Vec<u32> {
    //create empty stack
    let mut stack: Vec<u32> = Vec::with_capacity(max_size);
    stack
}
fn push(stack: &mut Vec<u32>, item: u32, max_size: usize) {
    if stack.len() == max_size {
        println!("cannot add more max size reach");
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack)
    }
}
fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let pop_value = stack.pop();
    pop_value
}
fn take_input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("there was an error");
    let input: u32 = input.trim().parse().expect("something was invalid");
    input
}
pub fn main() {
    println!("day 13");
    println!("pls enter the size of the stack");
    let max_size = take_input();
    let mut stack = create_stack(max_size as usize);

    println!("\n\n ************* MENU *************\n\n");
    println!("\n 1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
    println!("\n Enter your choice: ");
    let choice: u32 = take_input();
    match choice {
        1 => {
            println!("pls enter the value you want to insert");
            let value = take_input();
            push(&mut stack, value, max_size as usize);
        }
        2 => {
            println!("the elements that is poped is {:?}", pop(&mut stack))
        }
        3 => {
            println!("the Stack {:?}", stack)
        }
        4 => {
            println!("the size of stack is {:?}", stack.capacity())
        }
        _ => {}
    }
}
