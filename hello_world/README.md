# My Rust Learning Journey

## Environment Setup

Verified installed tools:

```
rustup 1.29.0 (28d1352db 2026-03-05)
rustc 1.96.0 (ac68faa20 2026-05-25)
cargo 1.96.0 (30a34c682 2026-05-25)
```

- **rustup** — manages Rust versions and toolchains
- **rustc** — the Rust compiler
- **cargo** — Rust's build system and package manager

---

## First Project: hello_world

### What I did

1. Navigated to my Rust folder:
   ```
   cd C:\Users\13112\Desktop\Rust
   ```

2. Tried to run `cargo build` inside a folder with just a `.rs` file — got this error:
   ```
   error: could not find `Cargo.toml` in `C:\Users\13112\Desktop\Rust\hello_world` or any parent directory
   ```
   **Why:** Cargo needs a `Cargo.toml` file to recognize a project. A lone `.rs` file isn't enough.

3. Fixed it by running inside the `hello_world` folder:
   ```
   cargo init
   ```

4. Built the project:
   ```
   cargo build
   ```

5. Ran the project:
   ```
   cargo run
   ```
   Output: `Hello, world!`

---

## Project Structure Explained

```
hello_world/
├── Cargo.toml      # Project config — name, version, dependencies
├── Cargo.lock      # Auto-generated — locks exact dependency versions
├── .gitignore      # Tells Git to ignore the target/ folder
├── src/
│   └── main.rs     # Your Rust code lives here
└── target/         # Compiled output — hello_world.exe is inside here
```

---

## Cargo Commands Explained

### `cargo init`

Turns an existing folder into a Cargo project. It creates:
- `Cargo.toml` — the project manifest
- `src/main.rs` — starter code with `Hello, world!`
- `.gitignore` — ignores the `target/` folder

Use it when you already have a folder and want to make it a Rust project.

### `cargo build`

Compiles your Rust code into an executable. It:
- Reads your `src/main.rs`
- Produces a `.exe` at `target/debug/hello_world.exe`
- Downloads and compiles any dependencies listed in `Cargo.toml`

The first build is slow. After that it only recompiles what changed.

**The difference from Python:** In Python you just run `python script.py` — no build step needed. In Rust, you must compile first (`cargo build`), then the `.exe` runs on its own without needing Rust installed.

### Common Commands

| Command | What it does |
|---|---|
| `cargo init` | Set up a new project in current folder |
| `cargo new name` | Create a brand new project folder |
| `cargo build` | Compile the code |
| `cargo run` | Compile + run in one step |
| `cargo check` | Check for errors without fully compiling (faster) |
| `cargo doc` | Generate HTML documentation for your project |
| `cargo doc --open` | Generate docs and open them in the browser |

---

## cargo doc

Running `cargo doc` generates a website-style HTML documentation page for your project from your code.

- Output is saved to `target/doc/hello_world/index.html`
- Use `cargo doc --open` to build and open in the browser automatically
- Right now it's mostly empty because `main.rs` has no doc comments yet

When you add `///` comments above functions, Cargo turns them into formatted docs:

```rust
/// Adds two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

This is similar to Python docstrings, but Cargo generates a full browsable website from them automatically. Dependencies you add also get their docs generated here so you can read them offline.

---

## Rust Basics Learned in main.rs

### Variables

```rust
let name = "Raj";
let age = 7;
```

Variables are declared with `let`. Rust infers the type automatically.

---

### Printing

`println!` prints and moves to the next line. `print!` stays on the same line.

```rust
println!("Hello, {}! from 🦀", name);  // prints with newline
print!("Never back down never what? "); // no newline — next print continues on same line
```

Use `\n` inside a string to manually add a new line:

```rust
print!("\nNever gonna give you up!\nNever gonna let you down!");
```

**`{}` is the placeholder** for values — like `f""` or `.format()` in Python:

```rust
println!("Once I was {} years old.", age);
// Python equivalent: print(f"Once I was {age} years old.")
```

---

### Comments

```rust
// single line comment

/*
   multi line comment
*/
```

---

### Mutable Variables

By default, variables in Rust are **immutable** (can't be changed). To allow changes, use `mut`:

```rust
let mut name = "Raj";
name = "Slim Shady"; // works because of mut
```

Without `mut`, trying to reassign throws a compile error. Python variables are always mutable — Rust forces you to be explicit.

---

### Data Types

Rust can infer types automatically, or you can declare them explicitly:

```rust
// inferred
let my_num = 5;
let my_double = 5.99;
let my_letter = 'D';
let my_bool = true;
let my_text = "Hello";

// explicit
let my_num: i32 = 5;
let my_double: f64 = 5.99;
let my_letter: char = 'D';
let my_bool: bool = true;
let my_text: &str = "Hello";
```

| Type | What it stores | Example |
|---|---|---|
| `i32` | Whole numbers, positive or negative | `5`, `-456` |
| `f64` | Decimal numbers | `5.99`, `3.14` |
| `char` | Single character — use single quotes | `'D'`, `'$'` |
| `bool` | True or false | `true`, `false` |
| `&str` | Text (string) — use double quotes | `"Hello"` |

**Key difference from Python:** Python figures out types at runtime and lets them change. Rust locks the type at compile time — a variable declared as `i32` can never hold text.

---

### Constants

Constants are like variables but can **never** change — not even with `mut`. They must always have an explicit type and by convention are written in ALL_CAPS:

```rust
const S: i32 = 7985;
println!("S will always be: {}", S);
```

- Must always have a type annotation (`: i32`, `: f64`, etc.)
- Must be uppercase — Rust will warn you if not
- Can be declared anywhere in the program, including outside functions
- Can never be reassigned, ever

**Python equivalent:** like a variable you just never reassign, but Rust enforces it at compile time.

---

### Booleans and If/Else

A boolean stores the result of a comparison — `true` or `false`:

```rust
let x: i32 = 6;
let y: i32 = 7;
let z: bool = x > y;
println!("Is {} greater than {}?\n{}", x, y, z); // prints false
```

Booleans are commonly used in `if/else` to decide what code runs:

```rust
if z {
    println!("{} is greater than {}", x, y);
} else {
    println!("{} is not greater than {}", x, y);
}
```

**Differences from Python:**
- No parentheses around the condition — `if z {` not `if (z):`
- Curly braces `{}` instead of indentation to define blocks
- No colon `:` after the condition

| Python | Rust |
|---|---|
| `if x > y:` | `if x > y {` |
| `    print("yes")` | `    println!("yes");` |
| `else:` | `} else {` |
| `    print("no")` | `    println!("no");` |
| *(end of indent)* | `}` |

### else if

Chain multiple conditions with `else if`:

```rust
let guess: i32 = 1;
if guess == S {
    println!("Its her");
} else if guess / S == 0 {
    println!("It might be her");
} else {
    println!("Its not her");
}
```

### if/else Must Return the Same Type

In Rust, if you use `if/else` to assign a value, both branches **must return the same type**. Mixing types causes a compile error:

```rust
// this works
let result = if number < 10 { "Too small" } else { "Big enough" };

// this FAILS — one branch is &str, the other is i32
let result = if number < 10 { "Too small" } else { 100 };
```

Python doesn't care about this — Rust does, because types are locked at compile time.

---

### Loops and Returning Values

In Rust, `while` and `for` loops **cannot** return a value — they always return `()` (nothing). Trying to assign their result causes an error.

Only `loop` can return a value, using `break`:

```rust
let mut number = 0;
let result = loop {
    number += 1;
    if number == 10 {
        break number; // returns number out of the loop
    }
};
println!("Stopped at: {}", result); // prints 10
```

| Loop type | Can return a value? |
|---|---|
| `while` | No — always returns `()` |
| `for` | No — always returns `()` |
| `loop` | Yes — via `break value` |

Python loops can never return a value directly — Rust's `loop` + `break value` is unique to Rust.

---

### Strings

Rust has two string types:

| Type | What it is | Example |
|---|---|---|
| `&str` | Fixed string slice — can't grow or change | `"Hello"` |
| `String` | Growable, owned string — can be modified | `String::from("Hello")` |

Create a `String` from text two ways:
```rust
let s1 = String::from("Hello");
let s2 = "Hello".to_string();
```

---

### Combining Strings

**Using `+`** — moves the left side, borrows the right:
```rust
let text1 = String::from("6");
let text2 = String::from("9");
let text3 = text1 + &text2; // text1 is gone after this, text2 still works
```

`&` before `text2` means borrow — lend it without giving ownership away. The `+` operator is defined as `String + &str` only. After `+`, `text1` is moved (consumed) and can't be used again.

**Using `format!` (recommended)** — nobody loses ownership:
```rust
let text1 = String::from("6");
let text2 = String::from("9");
let text4 = format!("{}{}", text1, text2); // both text1 and text2 still usable
```

**Using `.push_str()`** — append to a mutable string:
```rust
let mut text5 = String::from("6");
text5.push_str("9!"); // text5 is now "69!"
```

---

### `println!` vs `format!`

| | `println!` | `format!` |
|---|---|---|
| Prints to terminal | Yes | No |
| Returns a value | No — returns `()` | Yes — returns a `String` |

`println!` always returns `()` (nothing). You **cannot** assign it to a variable and use it:

```rust
// WRONG — forlen is (), has no .len()
let forlen = println!("hello");
println!("{}", forlen.len()); // error

// RIGHT — use format! to get a String back
let forlen = format!("hello");
println!("{}", forlen.len()); // works — prints 5
```

Use `format!` whenever you need to build a string to store or measure. Use `println!` just to print.

---

### String Differences from Python

Rust does **not** support Python-style string tricks:

| Python | Rust equivalent |
|---|---|
| `"-" * 100` | `"-".repeat(100)` |
| `"hello" + "world"` | cannot use `+` inside `println!` — write it as one string |

To print a repeated string:
```rust
println!("{}", "-".repeat(100));
// {} is needed because println! always requires a format string first
```

---

## What I Learned

- Rust and Cargo are separate tools but come bundled together via rustup
- Cargo is to Rust what npm is to Node, or pip is to Python
- Rust is compiled and strict — unlike Python, it won't run broken code
- The borrow checker is Rust's unique feature that enforces memory safety
- Rust errors happen at compile time, not at runtime like Python
- `cargo run` = build + run in one command
- `println!` needs `{}` as a placeholder — you can't drop values in directly
- Use `print!` instead of `println!` to stay on the same line
- String repetition is `.repeat(n)` not `* n` like Python
