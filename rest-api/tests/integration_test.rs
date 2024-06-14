use sqlx::PgPool;
use todo_api::config::DatabaseConfig;
use todo_api::db::*;

async fn connect_test_db() -> PgPool {
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

    let inserted_task = insert_todo(&connection_pool, "test".to_string(), "test".to_string())
        .await
        .unwrap();

    let retrieved_task = find_todo(&connection_pool, inserted_task).await.unwrap();

    assert_eq!(retrieved_task.title, "test");
    assert_eq!(retrieved_task.description, "test");
    assert_eq!(retrieved_task.completed, false);
}

#[tokio::test]
async fn update_todo_updates_record() {
    let connection_pool = connect_test_db().await;

    let inserted_task = insert_todo(&connection_pool, "test".to_string(), "test".to_string())
        .await
        .unwrap();

    update_todo(
        &connection_pool,
        inserted_task,
        "test 2".to_string(),
        "test 2".to_string(),
        true,
    )
    .await
    .unwrap();

    let retrieved_task = find_todo(&connection_pool, inserted_task).await.unwrap();

    assert_eq!(retrieved_task.title, "test 2");
    assert_eq!(retrieved_task.description, "test 2");
    assert_eq!(retrieved_task.completed, true);
}

#[tokio::test]
async fn delete_todo_removes_record() {
    let connection_pool = connect_test_db().await;

    let inserted_task = insert_todo(&connection_pool, "test".to_string(), "test".to_string())
        .await
        .unwrap();

    delete_todo(&connection_pool, inserted_task).await.unwrap();

    let result = find_todo(&connection_pool, inserted_task).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn list_todos_returns_items() {
    let connection_pool = connect_test_db().await;

    insert_todo(&connection_pool, "test".to_string(), "test".to_string())
        .await
        .unwrap();

    let task_list = list_todos(&connection_pool, 0, 10).await.unwrap();

    assert_ne!(task_list.items.len(), 0);
    assert_ne!(task_list.total_count, 0);
}
