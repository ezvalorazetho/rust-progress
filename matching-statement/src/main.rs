fn main() {
    let i = 6;

    match i {
        0_i32..=9_i32 => println!("I is a digit"),
        10_i32..=20_i32 => println!("I is two digit"),
        _ => println!("Unknown"),
    }
}
