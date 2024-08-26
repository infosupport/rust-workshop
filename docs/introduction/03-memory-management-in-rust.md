# Memory Management in Rust

In this section, we will explore the memory management model in Rust and how it differs from other programming languages.
Rust's ownership system ensures memory safety without the need for a garbage collector, making it a powerful and
efficient language for systems programming. We will cover concepts such as ownership, references, and borrowing, which
are fundamental to understanding how Rust manages memory. By the end of this section, you will have a solid
understanding of Rust's memory management model and be able to write efficient and safe code.

## Ownership in Rust

Ownership is a central concept in Rust that ensures memory safety without needing a garbage collector. Each value in 
Rust has a variable that's called its owner. There can only be one owner at a time, and when the owner goes out of scope,
the value will be dropped, freeing the memory. 

### Example

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is no longer valid here
    println!("{}", s2);
}
```

In this example, we create a new string and make `s1` the owner of that string. On the second line, we assign the value
of `s1` to the variable `s2`. We move the data to the new owner `s2`. The old variable `s1` is no longer the owner of the
string.

Note that you can't modify the value of a variable unless you declare it as mutable as the following example demonstrates:

```rust
let mut s1 = "Test".to_string();
s1 = "test2".to_string();
```

In the previous example, if we tried to assign a new value to `s1`, the compiler would tell us off. You can't modify
immutable variables. By declaring the same variable with the keyword `mut` we can overwrite the data.

## Rust's Ownership Model Compared to Other Languages

In languages like C and C++, the programmer is responsible for manually allocating and deallocating memory. This can
lead to memory leaks and other bugs. In languages like Java and Python, a garbage collector automatically manages memory,
but this can introduce performance overhead.

Rust's ownership model provides memory safety without needing a garbage collector. This means that Rust programs can
be as fast as C or C++ programs, but without the risk of memory leaks or other memory-related bugs.

## References and Borrowing

In Rust, you can create references to values without taking ownership. This is called borrowing. There are two types
of borrowing: immutable and mutable.

### Immutable References

You can have multiple immutable references to a value, but you cannot modify the value through these references. This
prevents a class of bugs related to race conditions. You can share immutable references across threads without worrying
someone will modify your data.

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}
```

### Mutable References

You can have only one mutable reference to a value at a time. This ensures that you cannot run into race conditions.
A mutable reference is created by adding the `mut` keyword to the reference.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    r1.push_str(", world");

    println!("{}", r1);
}
```

## Lifetime scopes

We already mentioned that when an owner of a piece of data goes out of scope, the data is cleaned up immediately. This 
poses some restrictions on how you can program. Rust works with lifetime scopes for each variable that's created.

You don't often need to work with explicit lifetime scopes, and for the purpose of this tutorial, we'll avoid having you
to think about them. But it's still useful to learn about them.

The standard or implicit lifetime scope is defined based on the lexical scope of a variable. If you declare a variable
like so:

```rust
fn my_function() {
    let my_variable = "test".to_string();
}
```

The life time scope for this variable is the scope of the function. Since we don't transfer ownership, the data is
cleaned up after we exit the scope of the function. Things get more complicated when you have a function like this:

```rust
fn my_function() -> String {
    let my_variable = "test".to_string();
    my_variable
}
```

In this example, the ownership is transferred to the parent scope where the function is called. Hence you have a lifetime
scope that's not determined by the function but by the parent where the function is called. It's good to know that it
works like this, but you don't have to worry about things going wrong here.

It gets more interesting when you want to keep a child property of a struct alive for longer than the struct that contains
the value for the property. You'll have to tell the compiler how to handle cleaning up the data by setting an explicit
lifetime scope. 

Discussions around lifetime can be quite complicated. If you're interested in learning more, we suggest checking
out the chapter about [Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html) in the rust manual.

## Summary

Rust's memory management model is based on ownership, which ensures memory safety without needing a garbage collector.
This makes Rust programs fast and safe. By understanding ownership, references, and borrowing, you can write efficient
and safe Rust code.

In the next section, [Pattern matching](./04-pattern-matching.md), we will discuss pattern matching in Rust. Pattern
matching is a powerful feature in Rust that allows you to match against the structure of values and perform different
actions based on the patterns. It makes Rust programs more concise, and is used by many Rust developers.