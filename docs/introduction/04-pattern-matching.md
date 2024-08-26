# Pattern Matching in Rust

Pattern matching is a powerful feature in Rust that allows you to match against the structure of data and destructure it in a concise and readable way. In this section, we will cover the following topics:

- What enums are, and how to create one
- How to pattern match against enums with the `match` syntax
- How to use pattern matching in if-statements
- How to deconstruct structs with pattern matching

## Enums in Rust

Enums, short for "enumerations," are a way to define a type that can have multiple variants. Each variant can have different data associated with it. Here's an example of how to define and create an enum in Rust:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, world!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
}
```

There are a few different types of enum variants that you can use:

1. If you don't need to associate any data, you can use just the name of the variation.
2. If you want to associate structured data, you can use an anonymous struct.
3. If you don't care about using fields, you can associate on or more values as shown in `Write(String)`, and `ChangeColor(i32, i32, i32)`.

Please check out [the documentation](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#defining-an-enum) to learn more about defining enums in Rust.

## Pattern Matching with `match`

The `match` syntax allows you to match against the different variants of an enum and perform different actions based on the variant. Here's an example:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
    }
}
```

## Pattern Matching in if-statements

You can also use pattern matching in if-statements to match against specific patterns. Here's an example:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    if let Message::Move { x, y } = msg {
        println!("Move to ({}, {})", x, y);
    } else {
        println!("Not a Move message");
    }
}
```

For more information on pattern matching in if-statements, refer to the [Rust book](https://doc.rust-lang.org/book/ch06-03-if-let.html).

## Deconstructing Structs with Pattern Matching

Pattern matching can also be used to deconstruct structs and extract their fields. Here's an example:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    let Point { x, y } = point;
    println!("Point coordinates: ({}, {})", x, y);
}
```

In this example, we deconstruct the `Point` struct and extract its `x` and `y` fields using pattern matching.

You can find more information about pattern matching in chapter 18 of the [Rust book](https://doc.rust-lang.org/book/ch18-00-patterns.html).

## Summary

In this section, we covered the powerful feature of pattern matching in Rust. Pattern matching allows you to match against the structure of data and destructure it in a concise and readable way. We explored the usage of pattern matching with enums, if-statements, and structs.

With enums, we learned how to define and create them, and then use the `match` syntax to match against the different variants of an enum and perform different actions based on the variant.
We also saw how pattern matching can be used in if-statements to match against specific patterns, providing a more expressive way to handle different cases.

Finally, we discovered that pattern matching can be used to deconstruct structs and extract their fields, making it easier to work with complex data structures.
By understanding and utilizing pattern matching in Rust, you can write more concise and readable code, taking full advantage of this versatile and powerful feature.

In the next section, [Working with structs and traits](./05-working-with-structs-and-traits.md), we'll dive int object oriented structures supported in Rust. We'll cover how you can create structs to model complex data structures and how to share behavior across data structures.
