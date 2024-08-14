# Module 4: Reading a configuration file

In this module, you'll learn how to read configuration for your application using the [config](https://docs.rs/config/latest/config/) crate.
This crate supports many different configuration sources and while we encourage you to try them out, we will use only one for this workshop.
We will use this to store the API key that the application needs to interact with the REST API.

For now, we will assume the configuration file lives in the current directory.
In a real-world scenario, consider using the [homedir crate](https://docs.rs/homedir/latest/homedir/) to reliably determine the user home directory - even on Windows!

## Prerequisites

You've completed [Module 3](./on-my-own-3.md).

## Instructions

1. Using the [config](https://docs.rs/config/latest/config/) crate, declare a configuration file **task.ini** that will be in [INI format](https://en.wikipedia.org/wiki/INI_file).
   You can use [`File::with_name`](https://docs.rs/config/latest/config/struct.File.html#method.with_name) and [`File::format`](https://docs.rs/config/latest/config/struct.File.html#method.format) for this.
2. Right after having prepared the logging, read the configuration file.
   Use [`Config::builder`](https://docs.rs/config/latest/config/struct.Config.html#method.builder) and [`ConfigBuilder::build`](https://docs.rs/config/latest/config/builder/struct.ConfigBuilder.html#method.build).
   If this fails, the program can [_panic_](https://doc.rust-lang.org/rust-by-example/std/panic.html) as we have no way of recovering from this.
3. Read the `apikey` setting from the `Config` value you just got from the configuration file.
   * Note that all `get_xxxx` methods on the `Config` trait return a `Result` value.
   This `Result` is a short-hand type for the standard `Result` type that comes with Rust.
   The short-hand represents either a success (`Ok`) or a failure (`Err`); the `Err` will always be a `ConfigError`, a type specific to the _config_ crate. 
   * Rusts [_pattern matching_](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) is an excellent way to deal with `Result` values.

   If the `apikey` setting is not there, print a clean error message and exit the program.
   If it is present, print its value.
   Of course, printing your API key is not something you would do in a real-world scenario.

Congratulations, you have made your first steps in processing application configuration!
