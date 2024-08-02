# Working with Structs and Traits

In this section, we will explore how to work with structs and traits in Rust. Structs are used to create custom data types, while traits are used to define shared behavior. By understanding these concepts, you can create more modular and reusable code in Rust.

## Structs

Structs in Rust are similar to classes in other languages like Java, C#, and Python. They are used to create custom data types that can hold multiple related values. Structs are defined using the `struct` keyword followed by the name of the struct and a set of curly braces containing the fields of the struct.

Here is an example of how to define and create a struct in Rust:

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Name: {}, Age: {}", person.name, person.age);
}
```

In this example, we define a struct called `Person` with two fields: `name` and `age`. We then create an instance of the `Person` struct and print its fields.

## Traits

Traits in Rust are similar to interfaces in other languages. They are used to define shared behavior that can be implemented by multiple types. Traits are defined using the `trait` keyword followed by the name of the trait and a set of curly braces containing the method signatures.

Here is an example of how to define and implement a trait in Rust:

```rust
trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        // &self refers to the current instance of the struct. Think of it as the this pointer in C# or Java.
        // Because we didn't declare self as mut, we can't change any data on the instance of Person here.
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
    };

    person.greet();
}
```

In this example, we define a trait called `Greet` with a single method `greet`. We then define a struct called `Person` and implement the `Greet` trait for the `Person` struct. Finally, we create an instance of the `Person` struct and call the `greet` method.

## Dynamic Dispatch

Rust also supports dynamic dispatch using trait objects. This allows you to use traits to achieve polymorphism similar to interfaces in other languages. For more information on dynamic dispatch, refer to the [Rust documentation](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html).

## Summary

In this section, we have covered the basics of working with structs and traits in Rust. Structs are used to create custom data types, and traits are used to define shared behavior. By using these features, you can create more modular and reusable code in Rust.
