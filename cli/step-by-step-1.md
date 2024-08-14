# Module 1: Getting Started

In this module, you'll learn how to bootstrap a new Rust project up till the point where you can print the words that our ancient traditions require us to print: "Hello, World".

## Prerequisites

* You have installed Rust using [rustup](https://rustup.rs/).
You can verify this works correctly by running `rustc -V` and `cargo -V`.
The output should look roughly similar to this
```console
$ rust -V
rustc 1.79.0 (129f3b996 2024-06-10)
$ cargo -V
cargo 1.79.0 (ffa9cf99a 2024-06-03)
```
* You have an editor set up to write your code. We recommend using [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Instructions

1. Create a new directory where you want the project to live.
Name it **task-cli**.
2. Inside this directory, create a file.
Name it **Cargo.toml** and fill it with the following content.
    ```toml
    [package]
    name = "task"
    edition = "2021"

    [dependencies]
    config = "0.14.0"
    clap = { version = "4.5.15", features = ["derive"] }
    log = "0.4.22"
    simplelog = "0.12.2"
    reqwest = "0.12.5"
    ```
3. Using a terminal (Bash, PowerShell or whatever you're comfortable with) navigate into the directory.
Issue the following command.
   ```console
   cargo run
   ```
    Notice how useful the error output is:
    ```console
    error: failed to parse manifest at `/tmp/task-cli/Cargo.toml`

    Caused by:
    no targets specified in the manifest
    either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
    ```
    This gives us a clear pointer for the next step.
4. Inside the directory, create a new directory.
Name it **src**.
5. Inside the **src** directory, create a new file.
Name it **main.rs** and fill it with the following content.
    ```rs
    pub fn main() {
        println!("Hello, World");
    }
    ```
6. Re-issue the `cargo run` command.
Notice that Cargo downloads a bunch of _crates_, packaged dependencies and compiles them.
As long as the dependencies don't change, this happens only once.
If all went well, the end of the output should look like this.
    ```console
    Compiling task v0.1.0 (/tmp/task-cli)
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
        Running `target/debug/task`
    Hello, World
    ```

Congratulations, you have bootstrapped your first Rust project!
[On to the next step: using a logging library](./step-by-step-2.md).