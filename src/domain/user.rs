pub struct NewUser {
    pub username: String,
    pub hashed_password: String,
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub hashed_password: String,
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, new_user: NewUser) -> Result<User, ()>;
    async fn update(&self, user: User) -> Result<User, ()>;
    async fn get_all(&self) -> Result<Vec<User>, ()>;
    async fn get_by_id(&self, id: i32) -> Result<Option<User>, ()>;
    async fn get_by_username(&self, username: &str)
    -> Result<Option<User>, ()>;
    async fn remove(&self, id: i32) -> Result<(), ()>;
}
