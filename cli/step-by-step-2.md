# Module 2: Configure logging

In this module, you'll learn how to add [a simple logging library](https://docs.rs/simplelog/latest/simplelog/) so we can write clean logging code later.

## Prerequisites

You've completed [Module 1](./step-by-step-1.md).

## Instructions

1. Open the **main.rs** file and add the following lines to the top:
   
   ```rust
   use simplelog::ColorChoice;
   use simplelog::LevelFilter;
   use simplelog::TermLogger;
   use simplelog::TerminalMode;
   ```

   We will be using data structures from the _simplelog_ crate.
   Now, just above the `main` function, add a new function that configures the _simplelog_ crate:

   ```rust
   fn prepare_logging(verbose: bool) {
       let level = if verbose {
           LevelFilter::Debug
       } else {
           LevelFilter::Info
       };
       let config = simplelog::Config::default();

       TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto).unwrap();
   }
   ```
   This code determines the logging level that our application will show, allowing us to suppress debug logging later.
   It also specifies how the logging will be outputted; in our case, it is printed to the terminal using standard out and standard error, using colors if possible.
3. Inside the main function, replace the call to the `println!` macro with the following block:

   ```rust
   prepare_logging(false);

   log::info!("Hello, world!");
   ```

4. Verify our application still works by running `cargo run` again.

Congratulations, you have configured logging in to your first Rust project!
Next step: [parsing command-line arguments](./step-by-step-3.md).
