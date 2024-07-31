# Memory Management in Rust

In this section, we will explore the memory management model in Rust and how it differs from other programming languages. Rust's ownership system ensures memory safety without the need for a garbage collector, making it a powerful and efficient language for systems programming. We will cover concepts such as ownership, references, and borrowing, which are fundamental to understanding how Rust manages memory. By the end of this section, you will have a solid understanding of Rust's memory management model and be able to write efficient and safe code.

## Ownership in Rust

Ownership is a central concept in Rust that ensures memory safety without needing a garbage collector. Each value in Rust has a variable that's called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped, freeing the memory.

### Example

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is no longer valid here
    println!("{}", s2);
}
```

In this example, `s1` is the owner of the string "hello". When we assign `s1` to `s2`, `s1` is no longer valid, and `s2` becomes the new owner.

## Rust's Ownership Model Compared to Other Languages

In languages like C and C++, the programmer is responsible for manually allocating and deallocating memory. This can lead to memory leaks and other bugs. In languages like Java and Python, a garbage collector automatically manages memory, but this can introduce performance overhead.

Rust's ownership model provides memory safety without needing a garbage collector. This means that Rust programs can be as fast as C or C++ programs, but without the risk of memory leaks or other memory-related bugs.

## References and Borrowing

In Rust, you can create references to values without taking ownership. This is called borrowing. There are two types of borrowing: immutable and mutable.

### Immutable References

You can have multiple immutable references to a value, but you cannot modify the value through these references.

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}
```

### Mutable References

You can have only one mutable reference to a value at a time. This ensures that you cannot have data races.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    r1.push_str(", world");

    println!("{}", r1);
}
```

## Summary

Rust's memory management model is based on ownership, which ensures memory safety without needing a garbage collector. This makes Rust programs fast and safe. By understanding ownership, references, and borrowing, you can write efficient and safe Rust code.

In the next section, [Pattern matching](./04-pattern-matching.md), we will discuss pattern matching in Rust. Pattern matching is a powerful feature in Rust that allows you to match against the structure of values and perform different actions based on the patterns. It makes Rust programs more concise, and is used by many Rust developers.