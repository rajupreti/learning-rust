// fn main() {
//     println!("Hello, world! from 🦀");
// }

fn main() {
    println!("{}", "-".repeat(100));
    let name = "Raj";
    println!("Hello, world! from {} in 🦀", name);
    println!("I am learning Rust!");
    println!("{}", "-".repeat(100));
    print!("Never back down never what? "); // this needs to be print instead of println to avoid moving to the next line
    print!("Never give up!"); // This will print on the same line as the previous print statement
    print!("\nNever gonna give you up!\nNever gonna let you down!"); // This will move to the next line after the previous print statements
    /*
    Get rickrolled b*tch
    */
    println!();
    println!("{}", "-".repeat(100));
    let name = "Lukas Graham";
    let age = 7;
    println!("Once I was {} years old.\nMy mama told me go make yourself some friends or you will be lonely song by {}", age, name);
    // cant use , or + to concatenate strings in Rust
    println!("{}", "-".repeat(100));
}

