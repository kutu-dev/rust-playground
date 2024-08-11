fn main() {
    let condition = true;
    let number = if condition { 7 } else { 3 };

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }
}
