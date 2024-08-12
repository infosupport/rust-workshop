# Module 2: Configure logging

In this module, you'll learn how to add a simple logging library so we can later write clean logging code.

## Prerequisites

You've completed [Module 1](./on-my-own-1.md).

## Instructions

1. Use [the _simplelog_ crate](https://docs.rs/simplelog/latest/simplelog/) to configure the way our application prints output:
    * Only "info" and up statements should be displayed to the user.
    * Output should normally go to standard out, but errors should go to standard error.
    * If the terminal supports it, it would be nice to have coloured output.
2. Inside the main function, use _simplelog_ instead of the `println` invocation to print the "Hello, world" at info level.
3. Verify our application still works by running `cargo run` again.
