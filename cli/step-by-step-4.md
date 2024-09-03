# Module 4: Reading a configuration file

In this module, you'll learn how to read application configuration using the [config](https://docs.rs/config/latest/config/) crate.
This crate supports many different configuration sources, and while we encourage you to try them out, we will use only one for this workshop.
We will use this to store the API key the application needs to interact with the REST API.

We will assume the configuration file lives in the current directory.
In a real-world scenario, consider using the [homedir crate](https://docs.rs/homedir/latest/homedir/) to reliably determine the user's home directory.

## Prerequisites

You've completed [Module 3](./step-by-step-3.md).

## Instructions

1. Change the **main.rs** file to include the _config_ crate by adding this block to the "use" statements:
   ```rs
   use config::Config;
   use config::File;
   use config::FileFormat;
   ```
   Next, add the following snippet to the `main` method below the `prepare_logging` invocation:
   ```rs
   let source = File::with_name("task.ini").format(FileFormat::Ini);
   let config = Config::builder()
       .add_source(source)
       .build()
       .unwrap();

   let api_key = match config.get_string("apikey") {
       Ok(api_key) => api_key,
       Err(error) => {
           log::error!("Can't find API key: {}", error.to_string());
           std::process::exit(1);
       }
   };

   log::info!("Found API key: {}", api_key);
   ```
   Let's break this down.
   1. The first line declares a configuration file named **task.ini**, following the [INI format](https://en.wikipedia.org/wiki/INI_file) in the working directory where our program is invoked.
   Note that the file is not read at this point - it is nothing more than a variable describing it.
   2. The second to fifth lines build a configuration by parsing this file.
   At this point, the file is read and converted into a `Config` value.
   Although `build` could return an error (its return type is `Result`, see below), we use `unwrap`.
   This will cause the program to panic (crash) if the `Result` is not OK, which is good.
   3. The remainder of the block processes the configuration.
   The line that starts with `let api_key = match` inspects the result of the `get_string` invocation, which is a `Result` value.
   This `Result` is a short-hand type for the standard `Result` type that comes with Rust.
   The short-hand represents either a success (`Ok`) or a failure (`Err`); the `Err` will always be a `ConfigError`, a type specific to the _config_ crate. 
   The `match` statement requires us to write _arms_ for each possible outcome - in this case, an `Ok` or an `Err` value, so we are sure we cover all cases.
   As `get_string` returns `Result<String>`, the `Ok` is guaranteed to have a `String` value, the `Err` will always have an `ConfigError`.
   4. The last line proves that we could read our configuration file.
   Of course, printing your API key is something other than what you would do in a real-world scenario.

Congratulations, you have made your first steps in processing application configuration!
Next step: [interacting with a remote REST API](./step-by-step-5.md).
