# Module 2: Configure logging

In this module, you'll learn how to add a simple logging library so we can later write clean logging code.

## Prerequisites

You've completed [Module 1](./step-by-step-1.md).

## Instructions

1. Open the **main.rs** file and add the following lines to the top:
   ```rs
   use simplelog::ColorChoice;
   use simplelog::LevelFilter;
   use simplelog::TermLogger;
   use simplelog::TerminalMode;
   ```
   This signals we will be using some data structures from the _simplelog_ crate.
   Now, just above the `main` function, add a new function that configures the _simplelog_ crate:
   ```rs
   fn prepare_logging(verbose: bool) {
       let level = if verbose { LevelFilter::Debug } else { LevelFilter::Info };
       let config = simplelog::Config::default();

       TermLogger::init(level, config, TerminalMode::Mixed, ColorChoice::Auto)
           .unwrap();
   }
   ```
   This code determines the logging level that our application will actually show, allowing us to suppress debug logging later.
   It also specifies how the logging is to be outputted; in our case, printed to the terminal using both standard out and standard error, using colours if possible.
2. Inside the main function, replace the call to the `println!` macro with the following block:
   ```rs
   prepare_logging(false);

   log::info!("Hello, world!");
   ```
3. Verify our application still works by running `cargo run` again.

Congratulations, you have configured logging in your first Rust project!
On to the next step: [parsing command-line arguments](./step-by-step-3.md)
