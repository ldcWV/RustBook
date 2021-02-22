fn main() {
    let tup: (i32, f64, u8) = (-123, 1.0, 4);
    let (x,_,_) = tup;
    println!("tup = ({},{},{})", tup.0, tup.1, tup.2);
    println!("x = {}", x);
}
