# Task management application

This application implements the REST API through which you can manage project tasks.
It's by no means complete, but it gives you a good idea of what programming with Rust looks like.

Please follow the instructions in this README file to understand how to operate this application.

## Configuring the environment

Make sure you have the following variables set in the `.env` file in the application directory:

| Variable name         | Description            | Example value |
|-----------------------| ---------------------- | ------------- |
| APP_DATABASE_HOST     | Database host          | localhost     |
| APP_DATABASE_PORT     | Database server port   | 5432          |
| APP_DATABASE_USERNAME | Database admin user    | postgres      |
| APP_DATABASE_PASSWORD | Database user password | postgres      |

## Running the application

Please use the following commands from the `rest-api` of the repository to run the application:

```shell
docker compose up -d
cargo run
```

These commands perform the following steps:

* The first command starts the database server with a database preconfigured.
* The second command runs the application.

## Testing the application

### Running unit-tests

Make sure you use the following command to run the unit-tests:

```shell
cargo test --lib
```

This command will only execute the unit-tests and skip over the integration tests.
For the integration tests, please check the next section.

### Running integration tests

Make sure you run `docker compose up -d` to get the test database running.

```shell
cargo test
```
