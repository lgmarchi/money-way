use crate::domain::category::{
    Category,
    CreateCategoryRequest,
};

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

    pub async fn get(&self, id: u64) -> Category {
        sqlx::query_as!(Category, "SELECT * FROM categories WHERE id = ?", id)
            .fetch_one(&self.db)
            .await
            .unwrap()
    }

    pub async fn create(
        &self,
        data: &CreateCategoryRequest,
        user_id: u64,
    ) -> Category {
        let query_result = sqlx::query!(
            "INSERT INTO categories (`user_id`, `name`, `description`) VALUES (?, ?, ?)",
            user_id,
            data.name,
            data.description,
        ).execute(&self.db).await.unwrap();

        self.get(query_result.last_insert_id()).await
    }
}
