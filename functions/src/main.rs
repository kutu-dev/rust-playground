fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    another_function(five(), 'h');
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function.");
    println!("x = {x}{unit_label}");
}
