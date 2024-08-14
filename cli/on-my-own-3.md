# Module 3: Parsing Command-Line Arguments

In this module, you'll learn how to process user input on the command line using the [clap](https://docs.rs/clap/latest/clap/) crate.
We will not cover writing an _interactive_ command-line application, but provide the basics, where the user needs to provide complete and correct inputs with each invocation.

## Prerequisites

You've completed [Module 2](./on-my-own-2.md).

## Instructions

1. Use the [_derive_ feature](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html) of the _clap_ crate to declare our initial command-line interface.
   It should be able to print its version, give a brief help information to users, and toggle a _verbose_ mode.
2. Let's verify our work so far.
   Open a terminal and navigate into the project directory, or re-use the terminal window you might have from module 1.
   Run `cargo run -- --help`; the `--` seperates arguments for `cargo run` from the arguments that we want to pass to our program; otherwise, `--help` would've been passed to `cargo run`.
   The output should provide a short manual for our future users!
3. Now let's use the information we get from our user to change application behaviour.
   Parse the command line arguments using `Cli::parse()`, then use the `verbose` flag to properly configure the logging library.
   If `verbose` is specified, the _simplelog_ crate should print "debug" and higher logging levels.
   If it isn't, "debug" statements should not be printed, only "info" and up.
   Now rebuild and play around with our newly created _verbosity_.