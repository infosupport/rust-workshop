# Learning the Basics of Rust

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
