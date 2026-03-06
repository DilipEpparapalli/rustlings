fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3; // It's basically shadowing the number. It is a new variable 'number'
    println!("Number plus two is: {}", number + 2);
}
