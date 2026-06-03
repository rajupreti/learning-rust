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
    println!("Once I was {} years old.\nMy mama told me go make yourself some friends or you will be lonely\n-song by {}", age, name);
    // cant use , or + to concatenate strings in Rust
    println!("{}", "-".repeat(100));
    // to change variables in Rust we need to use the mut keyword
    let mut name = "Raj";
    println!("My name is {}", name);
    name = "Slim Shady"; // if we try to change the value of name without using mut keyword it will throw an error
    println!("My name is {}", name);
    println!("{}", "-".repeat(100));
    // data types in Rust
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string
    println!("My number is {}", my_num);
    println!("My double(float) is {}", my_double);
    println!("My letter is {}", my_letter);
    println!("My boolean is {}", my_bool);
    println!("My text is {}", my_text);
    println!("{}", "-".repeat(100));
    // we can also specify the data types explicitly
    let my_num: i32 = 5;          // integer
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // string
    println!("My number is {}", my_num);
    println!("My double(float) is {}", my_double);
    println!("My letter is {}", my_letter);
    println!("My boolean is {}", my_bool);
    println!("My text is {}", my_text);
    /* 
    Numbers - Whole numbers and decimal numbers (i32, f64)
        The i32 type is used to store whole numbers, positive or negative (such as 123 or -456), without decimals
        The f64 type is used to store numbers containing one or more decimals
    Characters - Single letters or symbols (char)
        The char type is used to store a single character, such as 'a', '1', or '$'. Char values are surrounded by single quotes
    Strings - Text, a sequence of characters (&str)
        The &str type is used to store a sequence of characters (text). String values must be surrounded by double quote
    Booleans - True or false values (bool)
     */
    println!("{}", "-".repeat(100));
    const S: i32 = 7985; // constants are immutable by default and must be annotated with a type always be uppercase or warning
    println!("S will always be: {}", S);
}

