use askama::Template;

use crate::domain::task::{Task, TaskPriority, TaskStatus};

pub struct TaskViewModel {
    pub id: i32,
    pub description: String,
    pub priority: Option<TaskPriority>,
    pub priority_text: Option<String>,
    pub status: TaskStatus,
}

impl From<Task> for TaskViewModel {
    fn from(task: Task) -> Self {
        TaskViewModel {
            id: task.id,
            description: task.description.clone(),
            priority: task.priority.clone(),
            priority_text: task.priority.map(|p| format!("{:?}", p)),
            status: task.status.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "tasks.html")]
pub struct TasksTemplate {
    pub tasks: Vec<TaskViewModel>,
}
