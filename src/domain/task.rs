use serde::Deserialize;

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

#[derive(Deserialize, Clone, Debug)]
pub enum TaskStatus {
    #[serde(rename = "todo")]
    Todo,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
        }
    }
}

impl From<&str> for TaskStatus {
    fn from(s: &str) -> Self {
        match s {
            "todo" => TaskStatus::Todo,
            "in_progress" => TaskStatus::InProgress,
            "completed" => TaskStatus::Completed,
            _ => TaskStatus::Todo,
        }
    }
}

pub struct NewTask {
    pub description: String,
    pub priority: Option<TaskPriority>,
    pub status: TaskStatus,
}

pub struct Task {
    pub id: i32,
    pub description: String,
    pub priority: Option<TaskPriority>,
    pub status: TaskStatus,
}

impl NewTask {
    pub fn new(description: String, priority: Option<TaskPriority>) -> Self {
        Self {
            description,
            priority,
            status: TaskStatus::Todo,
        }
    }
}

impl Task {
    pub fn change_status(
        &mut self,
        new_status: TaskStatus,
    ) -> Result<&Self, ()> {
        self.status = new_status;
        Ok(self)
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
