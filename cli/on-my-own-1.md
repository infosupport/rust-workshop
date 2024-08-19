# Module 1: Getting Started

In this module, you'll learn how to bootstrap a new Rust project up till the point where you can print the words that our ancient traditions require us to print: "Hello, World".

## Prerequisites

* You have installed Rust using [rustup](https://rustup.rs/).
* You have an editor set up to write your code. We recommend using [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Instructions

1. Create a new directory where you want the project to live.
Name it **task-cli**.
2. Inside this directory, create a file.
Name it [**Cargo.toml**](https://doc.rust-lang.org/cargo/reference/manifest.html).
    * Fill the `[package]` section; at least `name` is required, and we suggest to also specify the `edition`  of Rust that we will be using: **2021**.
    * In the `[dependencies]` section, add dependencies to the following crates with their respective version numbers:
        * *config*: 0.14.0
        * *chrono*: 0.4.38, with the `serde` feature enabled
        * *clap*: 4.5.15, with the `derive` feature enabled
        * *log*: 0.4.22
        * *simplelog*: 0.12.2
        * *reqwest*: 0.12.6, with the `blocking` and `json` features enabled
3. Create a new file, **src/main.rs**.
Write a [function](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) named `main` and [invoke the `println` macro](https://doc.rust-lang.org/std/macro.println.html), passing "Hello, World" as its argument.
4. Run your program using `cargo run`.

Congratulations, you have bootstrapped your first Rust project!
[On to the next step: using a logging library](./on-my-own-2.md).