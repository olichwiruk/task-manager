use serde::Deserialize;

pub struct NewTask {
    pub description: String,
    pub priority: Option<TaskPriority>,
}

pub struct Task {
    pub id: i32,
    pub description: String,
    pub priority: Option<TaskPriority>,
}

#[derive(Deserialize, Clone, Debug)]
pub enum TaskPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskPriority::Low => "low",
            TaskPriority::Medium => "medium",
            TaskPriority::High => "high",
        }
    }
}

#[async_trait::async_trait]
pub trait TaskRepository: Send + Sync {
    async fn insert(&self, new_task: NewTask) -> Result<Task, ()>;
    async fn update(&self, task: Task) -> Result<Task, ()>;
    async fn get_all(&self) -> Result<Vec<Task>, ()>;
    async fn get_by_id(&self, id: i32) -> Result<Option<Task>, ()>;
    async fn remove(&self, id: i32) -> Result<(), ()>;
}
