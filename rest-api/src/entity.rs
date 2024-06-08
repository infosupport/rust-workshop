use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}