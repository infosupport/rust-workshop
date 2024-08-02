# Learning the Basics of Rust

## Learning the Basics of Rust

In this section, we will dive into the fundamentals of Rust programming. We will start by learning how to create a new project using the `cargo new` command and explore the basic file structure of a Rust project. Then, we will cover topics such as variables and mutability, data types including integers, booleans, and strings, and writing functions in Rust. Additionally, we will explore various control flow structures like `if`, `else`, `loop`, `while`, and `for`. These concepts form the foundation of Rust programming and will be essential as we continue our journey in learning Rust.

Let's get started with the basics of Rust programming!

## Creating a New Project with `cargo new`

To create a new Rust project, you can use the `cargo new` command. This command will create a new directory with the specified project name and set up the basic file structure for a Rust project.

```sh
cargo new my_project
```

This will create a directory called `my_project` with the following structure:

```
my_project
├── Cargo.toml
└── src
    └── main.rs
```

- `Cargo.toml`: This file contains metadata about your project, such as its name, version, and dependencies.
- `src/main.rs`: This is the main source file for your project. By default, it contains a simple "Hello, world!" program.

## Working with Variables and Mutability

In Rust, variables are immutable by default. This means that once a value is bound to a variable, it cannot be changed. To create a mutable variable, you can use the `mut` keyword.

```rust
fn main() {
    let x = 5; // immutable variable
    let mut y = 10; // mutable variable

    println!("x: {}, y: {}", x, y);

    y = 15; // changing the value of y
    println!("Updated y: {}", y);
}
```

## Data Types

Rust has several built-in data types, including integers, booleans, and strings.

### Integers

Rust supports various integer types, such as `i64` and `i32`. The number after the `i` indicates the number of bits used to store the value.

```rust
fn main() {
    let a: i64 = 1234567890123456789;
    let b: i32 = 123456789;

    println!("a: {}, b: {}", a, b);
}
```

### Booleans

The `bool` type represents a boolean value, which can be either `true` or `false`.

```rust
fn main() {
    let is_rust_awesome: bool = true;
    let is_rust_hard: bool = false;

    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is Rust hard? {}", is_rust_hard);
}
```

### Strings

Rust has two main string types: `String` and `&str`. The `String` type is a growable, heap-allocated string, while `&str` is a string slice that references a part of a string.

```rust
fn main() {
    let s1: String = String::from("Hello, Rust!");
    let s2: &str = "Hello, world!";

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

Working with `str` can be confusing at first. When I assign `"Hello, world!"` to `s2`, I'm actually doing two things. First, I allocate a chunk of memory containing a unicode string on the stack.
Then, I take a slice of the same size, and assign it to the variable `s2`.

The power of `str` becomes more apparent if you want to take parts of a string, for example the following code takes the first four letters of the string stored in `s2`:

```rust
let s3: &str = s2[0..5];
```

We took a slice of the string in the sample. The slice type is important in Rust and allows you to quickly and efficiently process binary and text information.
You can find more information about the slice type in the [Rust book](https://doc.rust-lang.org/book/ch04-03-slices.html).

## Writing a Function in Rust

Functions in Rust are defined using the `fn` keyword, followed by the function name, parameters, and return type (if any).

```rust
fn main() {
    let result = add(5, 10);
    println!("Result: {}", result);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

## Control Flow Structures

Rust provides several control flow structures, such as `if`, `else`, `loop`, `while`, and `for`.

### If-Else

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is 5 or greater");
    }
}
```

### Loop

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            break;
        }
    }
}
```

### While

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!");
}
```

### For

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}
```

## Summary

In this section, we learned about the basics of Rust programming. We saw how to create a new project using the `cargo new` command and explored the file structure of a Rust project. We also learned about variables and mutability in Rust, and how to work with different data types such as integers, booleans, and strings. Additionally, we saw how to define functions in Rust and explored various control flow structures like `if`, `else`, `loop`, `while`, and `for`. These concepts form the foundation of Rust programming and will be essential as we continue our journey in learning Rust.

In the next section,[Memory management in Rust](./03-memory-management-in-rust.md), we will explore the topic of memory management in Rust. This section will cover important concepts such as ownership, borrowing, and lifetimes in Rust. We will learn how Rust's ownership system helps prevent common memory-related bugs like null pointer dereferences and memory leaks. Additionally, we will understand how to use references and borrowing to safely share and manipulate data in Rust. This section will provide a solid foundation for understanding Rust's unique approach to memory management and will be crucial for writing safe and efficient Rust code.
