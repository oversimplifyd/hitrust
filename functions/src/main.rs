fn main() {
    // statement
    let first_result = add_one();

    let second_result = add_one_more(36);

    println!("Results: {}, {}, {}", first_result, second_result, add_one_with_return());
}

fn add_one() -> u32 {
    //Expressions are expected at the end of a function or before it ends 
    // adding semicolon to this will make it a statement and the calling program will err 
    // because Rust will return an empty tuple since it could't find a returned expression
    6 + 1
}

fn add_one_more(number: u32) -> u32 {
   return add_one() + number + 1
}

fn add_one_with_return() -> i32 {
    let num: i32 = 3;

    if num > 3 {
        return 3
    }

    num + 1
}
