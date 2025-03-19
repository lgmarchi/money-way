use crate::domain::category::Category;

pub struct CategoryRepository {
    db: sqlx::MySqlPool,
}

impl CategoryRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }

    pub async fn get_all_of_user(&self, user_id: u64) -> Vec<Category> {
        sqlx::query_as!(
            Category,
            "SELECT * FROM categories WHERE user_id = ?",
            user_id
        )
        .fetch_all(&self.db)
        .await
        .unwrap()
    }
}

pub async fn get_all_of_user(
    db: &sqlx::MySqlPool,
    user_id: u64,
) -> Vec<Category> {
    sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE user_id = ?",
        user_id
    )
    .fetch_all(db)
    .await
    .unwrap()
}
