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

    println!("{}", "-".repeat(100));
    let x: i32 =  6;
    let y: i32 = 7;
    let z: bool = x > y;
    println!("Is {} greater than {}?\n{}", x, y, z);
    // Boolean values are often used in if statements to decide what code should run
    println!("According to the if else statement:");
    if z {
        println!("{} is greater than {}", x, y);
    } else {
        println!("{} is not greater than {}", x, y);
    }
    println!("{}", "-".repeat(100));
    let guess: i32 = 1;
    if guess == S {
        println!("Its her");
    } else if guess / S == 0 {
        println!("It might be her");
    } else {
        println!("Its not her");
    }
    /*
    The value from if and else must be the same type, like two pieces of text or two numbers.
    When you mix types, like a string and an integer, you'll get an error:
    let number = 5;
    let result = if number < 10 { "Too small" } else { 100 };
    println!("{}", result);
    */
    println!("{}", "-".repeat(100));
    // rust matches work like switch statements in c++
    let day = 5;
    match day {
        1 => println!("I dont wanna go to the office day"),
        2 => println!("I think I have to go to the office day"),
        3 => println!("Weekend in 2 days day"),
        4 => println!("Lets get drunk and take télétraviail tomorrow day"),
        5 => println!("F*ck it, its the almost weekend we will see Monday day"),
        6 => println!("I slept all day day"),
        7 => println!("Its sunday already, should I call in sick tomorrow day"),
        _ => println!("Invalid day"), // _ is used to catch all other values that are not matched by the previous patterns
    }
    println!("{}", "-".repeat(100));

}

