use serde::Deserialize;

#[derive(Deserialize)]
pub enum TaskPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

#[derive(Deserialize)]
pub struct TaskForm {
    pub description: String,
    pub priority: TaskPriority,
}
