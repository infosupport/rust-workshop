# Module 3: Parsing Command-Line Arguments

In this module, you'll learn how to process user input on the command line using the [clap](https://docs.rs/clap/latest/clap/) crate.
We will not cover writing an _interactive_ command-line application, but provide the basics, where the user needs to provide complete and correct inputs with each invocation.

## Prerequisites

You've completed [Module 2](./step-by-step-2.md).

## Instructions

1. Change the **main.rs** file to include the _clap_ crate by adding this line to the block of "use" statements:
`use clap::Parser;`.
   Next, declare the first command-line switches and commands by writing this snippet of code above the `prepare_logging` method:
   ```rs
   #[derive(Parser)]
   #[command(version, about, long_about = None)]
   struct Cli {
   }
   ```
   Because we used `features = ["derive"]` in **Cargo.toml**, we can leverage the _declarative_ mode of the _clap_ crate.
   This means we write a _struct_ (a named set of values that occupy a block of memory) and rely on clap to provide us with a populated struct at runtime.
   The clap crate can do this because we added a _derive_ attribute on our _struct_, and we pass it the `Parser` macro that will generate the complete argument parser at build-time.
   We also add a few common commands to the application by applying the `command` macro; this macro will add a `version` and a `help` option.
2. Let's verify our work so far.
   Open a terminal and navigate into the project directory, or re-use the terminal window you might have from module 1.
   Run `cargo run -- --help`; the `--` seperates arguments for `cargo run` from the arguments that we want to pass to our program; otherwise, `--help` would've been passed to `cargo run`.
   The output should provide a short manual for our future users!
3. Let's add a global flag to our application.
   Inside the `Cli` struct, add these two lines:
   ```rs
    #[arg(short = 'v', long = "verbose", help = "Enables verbose mode")]
    verbose: bool,
   ```
   This declares a flag that can be enabled or disabled, with a "short" toggle (`-v`) as well as a longer one (`--verbose`).
   Now, inside the `main` function, just above the call to `prepare_logging`, add the following snippet:
   ```rs
   let cli = Cli::parse();
   let verbose = cli.verbose;
   ```
   These two lines invoke the parser that the `Parser` macro has generated for us, and reads the outcome.
   Let's use that to configure our logging; replace the current invocation of `prepare_logging` with
   ```rs
   prepare_logging(verbose);
   ```
   And just above the call to `log::info`, add this line
   ```rs
   log::debug!("Test");
   ```
   Now rebuild and play around with our newly created _verbosity_.

Congratulations, you have made your first steps in parsing command-line arguments!
On to the next step: [reading a configuration file](./step-by-step-4.md).