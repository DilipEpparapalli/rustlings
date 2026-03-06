// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; // by default, variable in rust are immutable, all we have to do is make them mutable by adding keyword 'mut'
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
