fn main() {
    another_function(5, 6);
    let x = five();
    println!("x = {}", x);
}

fn five() -> i32 {
    5
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
