//! This module contains a set of integration tests to verify that the database interactions work as intended.
//!
//! We made the assumption that you have PostgreSQL running on your local machine with a database that contains the todos
//! table as defined in the sql folder in the root of the repository. If you don't have the database set up, you can
//! use the docker-compose file in the root of the repository to start a postgres instance.
//!
//! You can run the tests using the following command:
//!
//! ```sh
//! cargo test
//! ```
//!
//! We recommend excluding these tests from the default test suite because they require a running database instance.
//! You can do this by running the tests with the following command:
//!
//! ```sh
//! cargo test --lib
//! ```

use dotenv::dotenv;
use sqlx::PgPool;
use todo_api::config::DatabaseConfig;
use todo_api::db::*;

async fn connect_test_db() -> PgPool {
    dotenv().ok();

    let db_config = DatabaseConfig {
        host: std::env::var("DB_HOST").unwrap().to_string(),
        port: std::env::var("DB_PORT").unwrap().parse().unwrap(),
        name: std::env::var("DB_NAME").unwrap().to_string(),
        username: std::env::var("DB_USER").unwrap().to_string(),
        password: std::env::var("DB_PASSWORD").unwrap().to_string(),
    };

    connect_db(&db_config).await.unwrap()
}

#[tokio::test]
async fn insert_todo_creates_record() {
    let connection_pool = connect_test_db().await;

    let inserted_task = insert_task(
        &connection_pool,
        1,
        "test".to_string(),
        "test description".to_string(),
    )
    .await
    .unwrap();

    let retrieved_task = find_task(&connection_pool, 1, inserted_task).await.unwrap();

    assert_eq!(retrieved_task.title, "test");
    assert_eq!(retrieved_task.description, "test description");
    assert_eq!(retrieved_task.completed, false);
}

#[tokio::test]
async fn update_todo_updates_record() {
    let connection_pool = connect_test_db().await;

    let inserted_task = insert_task(
        &connection_pool,
        1,
        "test".to_string(),
        "test description".to_string(),
    )
    .await
    .unwrap();

    update_task(
        &connection_pool,
        1,
        inserted_task,
        "test 2".to_string(),
        "test description 2".to_string(),
        true,
    )
    .await
    .unwrap();

    let retrieved_task = find_task(&connection_pool, 1, inserted_task).await.unwrap();

    assert_eq!(retrieved_task.title, "test 2");
    assert_eq!(retrieved_task.description, "test description 2");
    assert_eq!(retrieved_task.completed, true);
}

#[tokio::test]
async fn delete_task_removes_record() {
    let connection_pool = connect_test_db().await;

    let inserted_task = insert_task(
        &connection_pool,
        1,
        "test".to_string(),
        "test description".to_string(),
    )
    .await
    .unwrap();

    delete_task(&connection_pool, 1, inserted_task)
        .await
        .unwrap();

    let result = find_task(&connection_pool, 1, inserted_task).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn list_task_returns_items() {
    let connection_pool = connect_test_db().await;

    insert_task(&connection_pool, 1, "test".to_string(), "test".to_string())
        .await
        .unwrap();

    let task_list = list_tasks(&connection_pool, 1, 0, 10).await.unwrap();

    assert_ne!(task_list.items.len(), 0);
    assert_ne!(task_list.total_count, 0);
}
