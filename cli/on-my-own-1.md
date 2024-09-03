# Module 1: Getting Started

In this module, you'll learn how to bootstrap a new Rust project up till the point where you can print the words that our ancient traditions require us to print: "Hello, World".

## Prerequisites

* You have installed Rust using [rustup](https://rustup.rs/).
* You have an editor set up to write your code. We recommend using [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Instructions

1. Create a new directory where you want the project to live.
Name it **task-cli**.
2. Inside this directory run `cargo init .` to initialize the new project.
3. Add the following dependencies to your project using `cargo add <dependency>` (Use `cargo add --help` if you want to learn how to use features of packages)

  | Dependency | Features |
  | ---------- | -------- |
  | chrono | serde |
  | clap | derive |
  | config | | 
  | log | |
  | reqwest | json, blocking |
  | serde | derive |
  | simplelog |

4. Open, **src/main.rs**.
Write a [function](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) named `main` and [invoke the `println` macro](https://doc.rust-lang.org/std/macro.println.html), passing "Hello, World" as its argument.
5. Run your program using `cargo run`.

Congratulations, you have bootstrapped your first Rust project!
[On to the next step: using a logging library](./on-my-own-2.md).
