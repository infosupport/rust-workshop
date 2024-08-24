# Module 5: Implement the First API call

In this module, you'll learn how to perform an HTTP request using the [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate.
This crate provides a nice, high-level API client.
It supports both synchronous (blocking) and asynchronous (non-blocking) calls.
For sake of simplicity, we will be using the blocking calls.

## Prerequisites

You've completed [Module 4](./on-my-own-4.md).

## Instructions

1. Create a new file in the same directory as before and name it **api.rs**.
2. Inside **api.rs**, we define a Rust _struct_ to hold the data we need for interacting with the remote API:
   * the HTTP client itself, of type [`reqwest::blocking::Client`](https://docs.rs/reqwest/latest/reqwest/struct.Client.html), 
   * the remote host name and
   * the API key that we read in the previous module.

   Note that we not only keep the API key in memory, but also the HTTP client, because the Reqwest documentation says
   >  If you plan to perform multiple requests, it is best to create a `Client` and reuse it, taking advantage of keep-alive connection pooling.
   
   Next, we want to be able to instantiate a copy of that struct.
   Add the following snippet to **api.rs**:
   ```rs
   impl ApiClient {
       pub fn new(api_key: String) -> Self {
           // TODO return an instance of the ApiClient struct
       }
   }
   ```
3. Before we go any further, we want to define the operations that our API client can perform.
   They must match the [REST API operations](../rest-api/src/web.rs) that our server offers; see its `create_router` function for the full list.

   1. Since their return types must match those of the REST API, copy the `PagedResult` _struct_ and the `Task` struct over from [the servers **entity.rs**](https://github.com/infosupport/rust-workshop/blob/main/rest-api/src/entity.rs) to a new file in your project, but remove the `FromRow` macros from the `derive` attributes.
   2. Write the signature for the Rust method that will correspond with the first API call, `GET /v1/todos`.
   Do that in a Rust _trait_, which is some kind of an _interface_: it defines a function that must exist on any data structure that has the `TaskApiClient` _trait_.
   The function will have to return a [`Result<PagedResult<Task>, reqwest::Error`](https://doc.rust-lang.org/std/result/index.html).
   Normally, you don't want to leak your library types into your application code base, but for now we'll stick with it.
4. Now that the behaviour is well defined, we can start implementing it.
   After the `impl ApiClient` block, create a new, similar block: `impl TaskApiClient for ApiClient`.
   This will contain the code that allows the `ApiClient` _struct_ to have the `TaskApiClient` _trait_.
   Put differently, it makes the `ApiClient` implement the `TaskApiClient` interface that we just wrote.
   Using the Reqwest crate, and the `reqwest::blocking::Client` struct, write an implementation.
   Don't forget to include the `X-Api-Key` in the HTTP GET call!

   When it comes to handling the response, the [`error_for_status()` method](https://docs.rs/reqwest/latest/reqwest/blocking/struct.Response.html#method.error_for_status) can be useful.
   From an HTTP point of view, every call that gets a response is a `Ok`, no matter what the response was: a `404 NOT FOUND` is just as good as a `200 OK`.
   The `error_for_status` method changes this and turns all responses with status code between 400 and 599 in an `Error`.
5. Finally, inside the **main.rs** file, use your new API client to invoke the REST API and print something useful from the information that you retrieve with it!


Congratulations, you've made it through the hardest part of this workshop!
From here on, you have seen all the bricks to start building your own application.
