use sqlx::PgPool;

use crate::domain::task::{NewTask, Task, TaskPriority, TaskRepository};

pub struct SqlxRepository {
    pool: PgPool,
}

impl SqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskRepository for SqlxRepository {
    async fn insert(&self, new_task: NewTask) -> Result<Task, ()> {
        let rec = sqlx::query!(
            "INSERT INTO tasks (description, priority, status) VALUES ($1, $2, $3) RETURNING id, description, priority, status",
            new_task.description,
            new_task.priority.as_ref().map(|p| p.as_str()),
            new_task.status.as_str()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ())?;

        Ok(Task {
            id: rec.id,
            description: rec.description,
            priority: rec.priority.and_then(|p| match p.as_str() {
                "low" => Some(TaskPriority::Low),
                "medium" => Some(TaskPriority::Medium),
                "high" => Some(TaskPriority::High),
                _ => None,
            }),
            status: new_task.status,
        })
    }

    async fn update(&self, task: Task) -> Result<Task, ()> {
        let rec = sqlx::query!(
            "UPDATE tasks SET description = $2, priority = $3, status = $4 WHERE id = $1 RETURNING id, description, priority, status",
            task.id,
            task.description,
            task.priority.as_ref().map(|p| p.as_str()),
            task.status.as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ())?;

        Ok(Task {
            id: rec.id,
            description: rec.description,
            priority: rec.priority.and_then(|p| match p.as_str() {
                "low" => Some(TaskPriority::Low),
                "medium" => Some(TaskPriority::Medium),
                "high" => Some(TaskPriority::High),
                _ => None,
            }),
            status: task.status,
        })
    }

    async fn get_all(&self) -> Result<Vec<Task>, ()> {
        let records = sqlx::query!(
            "SELECT id, description, priority, status FROM tasks ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| ())?;

        Ok(records
            .into_iter()
            .map(|rec| Task {
                id: rec.id,
                description: rec.description,
                priority: rec.priority.and_then(|p| match p.as_str() {
                    "low" => Some(TaskPriority::Low),
                    "medium" => Some(TaskPriority::Medium),
                    "high" => Some(TaskPriority::High),
                    _ => None,
                }),
                status: rec.status.as_str().into(),
            })
            .collect())
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Task>, ()> {
        let record = sqlx::query!(
            "SELECT id, description, priority, status FROM tasks WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ())?;

        if let Some(record) = record {
            Ok(Some(Task {
                id: record.id,
                description: record.description,
                priority: record.priority.and_then(|p| match p.as_str() {
                    "low" => Some(TaskPriority::Low),
                    "medium" => Some(TaskPriority::Medium),
                    "high" => Some(TaskPriority::High),
                    _ => None,
                }),
                status: record.status.as_str().into(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn remove(&self, id: i32) -> Result<(), ()> {
        sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|_| ())?;

        Ok(())
    }
}
