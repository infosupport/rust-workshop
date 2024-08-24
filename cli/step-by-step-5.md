# Module 5: Implement the First API call

In this module, you'll learn how to perform an HTTP request using the [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate.
This crate provides a nice, high-level API client.
It supports both synchronous (blocking) and asynchronous (non-blocking) calls.
For sake of simplicity, we will be using the blocking calls.

## Prerequisites

You've completed [Module 4](./step-by-step-4.md).

## Instructions

1. Create a new file in the same directory as before and name it **api.rs**.
2. Inside **api.rs**, we will first define a Rust _struct_ to hold the data we need for interacting with the remote API.
   Think of this _struct_ as all the variables and data our application needs to interact with the REST API:
   * the HTTP client itself, 
   * the remote host name and
   * the API key that we read in the previous module.

   Add the following lines to **api.rs**:
   ```rs
   pub struct ApiClient {
       api_key: String,
       host_name: String,
       http_client: reqwest::blocking::Client,
   }
   ```
   Note that we not only keep the API key in memory, but also the HTTP client, because the Reqwest documentation says
   >  If you plan to perform multiple requests, it is best to create a `Client` and reuse it, taking advantage of keep-alive connection pooling.
   
   Next, we want to be able to instantiate a copy of that struct.
   Add the following snippet to **api.rs**:
   ```rs
   impl ApiClient {
       pub fn new(api_key: String) -> Self {
           ApiClient {
               api_key: api_key,
               host_name:  "http://localhost:3000".to_string(),
               http_client: reqwest::blocking::Client::new(),
           }
       }
   }
   ```
   You can interpret this as a _constructor_ for the struct: it stores the API key, the hostname where the REST API is running and a new instance of the `reqwest::blocking::Client` together.
3. Before we go any further, we want to define the operations that our API client can perform.
   They must match the [REST API operations](../rest-api/src/web.rs) that our server offers; see its `create_router` function for the full list.

   1. First, their return types must match those of the REST API.
      To achieve that, create a new file, **model.rs** in a folder named **api** under **src**.
      Copy the `PagedResult` _struct_ and the `Task` struct over from [the servers **entity.rs**](https://github.com/infosupport/rust-workshop/blob/main/rest-api/src/entity.rs) (lines 14 till 48) over to this new file.
      Drop the `FromRow` macro from the `derive` attribute.
      It is necessary for reading the struct from the database, so we don't need it for this client.
   2. Next, we must write the signature for the Rust method that will correspond with the first API call, `GET /v1/todos`.
      To do that, we add a Rust _trait_ to the **api.rs** file:
      ```rs
      pub trait TaskApiClient {
        fn list(&self, page_num: u8) -> Result<PagedResult<Task>, reqwest::Error>;
      }
      ```
      Think of this as an _interface_: it defines a function that must exist on any data structure that has the `TaskApiClient` _trait_.
      It defines a function that needs a borrow of "self" ("this" in many other languages) and a page number, which will be an unsigned integer of 8 bits; this allows for values 0 - 255, which is probably enough for this workshop.
      That function will then return a [`Result<PagedResult<Task>, reqwest::Error`](https://doc.rust-lang.org/std/result/index.html).
      `Result`, again, is an enum that is either an `Ok()` (in this case, with a `PagedResult` of `Task` in it), or an `Err`, in which case it will hold an `Error` from the Reqwest crate.
      Normally, you don't want to leak your library types into your application code base, but for now we'll stick with it.
4. Now that the behaviour is well defined, we can start implementing it.
   After the `impl ApiClient` block, create a new, similar block: `impl TaskApiClient for ApiClient`.
   This will contain the code that allows the `ApiClient` _struct_ to have the `TaskApiClient` _trait_.
   Put differently, it makes the `ApiClient` implement the `TaskApiClient` interface that we just wrote.
   Using the Reqwest crate, we can write the implementation:
   ```rs
   fn list(&self, page_num: u8) -> Result<PagedResult<Task>, reqwest::Error> {
       let url = format!("{}/v1/todos?page={}", &self.host_name, page_num);
       log::debug!("GET {}", url);
       let response = self
           .http_client
           .get(url)
           .header("X-Api-Key", &self.api_key)
           .send();

       // Placeholder

       return result;
   }
   ```
   This first part performs the actual HTTP call.
   It takes the hostname where the API is running, builds the complete URL from it, performs an HTTP `GET` request against that URL with an additional `X-Api-Key` header.
   
   Now that the HTTP call is out and answered, we must do something with the response.
   Replace the `// Placeholder` with the following piece of code:
   ```rs
   match response?.error_for_status() {
       Ok(body) => {
           log::debug!("Looks good so far!");
           body.json::<PagedResult<Task>>()
       }
       Err(error) => {
           log::error!(
               "Remote API at {} returned response with status {}",
               error.url().unwrap(),
               error.status().unwrap()
           );
           Err(error)
       }
    };
   ```
   The `response` variable is of type `Result<Response, Error>`.
   From an HTTP point of view, every call that gets a response is a `Ok`, no matter what the response was: a `404 NOT FOUND` is just as good as a `200 OK`.
   The `error_for_status` method changes this and turns all responses with status code between 400 and 599 in an `Error`.
   We can then match against the outcome of that:
   1. The `Ok(body)` arm of the match runs when the result of `error_for_status` is (still) an `Ok` value, and creates a local variable `body` with the actual response.
   The `json::<PagedResult<Task>>()` call uses the Serde crate and our `Deserialize` macro to parse the response body into instances of the `PagedResult` and `Task` structs.
   2. The `Err(error)` arm will run when the result of `error_for_status` is an `Err` struct, and makes its inner error available in the `error` variable.
   We use it to print a few things - the `unwrap()` invocations are there because not all Reqwest errors will have an HTTP status code, and because you could chose to remove the URL from an error if you suspect it to contain sensitive information. 

   Note also how the last statements inside each arm do not end with a semicolon!
   This automatically makes those expressions the return value of that arm.
   Since the `match` is exhaustive (it has to be, the Rust compiler will print an error if it isn't), we can use the same trick for the `match` itself - no semicolon or return and it automatically becomes the return value for this function.

5. Finally, change the **main.rs** file to use this new REST API client and print something useful from the information that we retrieve with it.
   Replace the last lines of the `main` function with this:
   ```rs
   let client = api::ApiClient::new(api_key);
   ```
   This instantiates the struct _with_ the trait that will help us interact with the REST API.
   Next, let's do the first API call using this client:
   ```rs
   match client.list(0) {
       Ok(result) => {
           log::info!(
               "Found {} tasks, displaying first {}",
               result.total_count,
               min(i64::from(result.page_size), result.total_count)
           );
           for task in result.items {
               let created = task.date_created.format("%B %d, %Y at %H:%M");
               log::info!("{:>4}: {} [created {}]", task.id, task.title, created)
           }
       }
       Err(e) => {
           log::error!("Can't invoke API: {}", e);
           std::process::exit(1);
       }
   }
   ```
   Let's break this down.
   1. The first line does the actual invocation and request the first page - which is indicated with the magic number `0`.
   The result of that function call is then matched - are you starting to ee a pattern? - against two arms.
   2. The `Ok` arm logs how many tasks are in the result and prints a summary for each of them.
   Because `page_size` is a relatively small number it fits in an `i32` (32-bit integer).
   The number of tasks one can have (`total_count`) is much larger, so it's declared as an `i64` (64-bit integer).
   The `min` method only works on variables of the same type, so we must "upcast" `page_size` to also be an `i64`.
   3. The `Err` arm just prins the error it found and then terminates the program with exist code `1`.
   It is customary that non-succesfull termination of a program is signaled by a non-zero exit code.

Congratulations, you've made it through the hardest part of this workshop!
From here on, you have seen all the bricks to start building your own application.
