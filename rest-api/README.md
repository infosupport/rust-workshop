# TODO Application

This application implements the REST API through which you can manage simple TODO items.
It's by no means complete, but it gives you a good idea of what programming with Rust looks like.

Please follow the instructions in this README file to understand how to operate this application.

## Running the application

Please use the following commands from the root of the repository to run the application:

```shell
docker compose up -d
cd rest-api
cargo run
```

These commands perform the following steps:

* The first command starts the database server with a database preconfigured.
* The second command changes the directory to the rest-api directory.
* The final command runs the application.

## Testing the application

### Running unit-tests

Make sure you use the following command to run the unit-tests:

```shell
cargo test --lib
```

This command will only execute the unit-tests and skip over the integration tests.
For the integration tests, please check the next section.

### Running integration tests

To run integration tests, run these instructions:

```shell
cargo test
```

Before you do, make sure you have a postgres database running.
You can use `docker compose up -d` to get one running quickly.

You need to set the following environment variables:

| Variable name | Description            | Example value |
| ------------- | ---------------------- | ------------- |
| DB_HOST       | Database host          | localhost     |
| DB_PORT       | Database server port   | 5432          |
| DB_USER       | Database admin user    | postgres      |
| DB_PASSWORD   | Database user password | postgres      |

If you're manually configuring PostgreSQL you'll need to run
the `../sql/00-create-db.sql` script to get the right structure.

Once you've configured a PostgreSQL server with a database, you can run the 
integration tests with the following command:

```shell
cargo test
```

### Manual testing

You can use the scripts in `../tests/` to run manual tests if you have the 
[REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension in VSCode.
