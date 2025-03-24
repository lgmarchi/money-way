use crate::domain::transaction::{
    CreateTransactionRequest,
    Transaction,
    UpdateTransactionRequest,
};

pub struct TransactionRepository {
    db: sqlx::MySqlPool,
}

impl TransactionRepository {
    pub fn new(db: sqlx::MySqlPool) -> Self {
        Self { db }
    }

    pub async fn get_all_of_user(&self, user_id: u64) -> Vec<Transaction> {
        sqlx::query_as!(
            Transaction,
            "SELECT * FROM transactions WHERE user_id = ?",
            user_id
        )
        .fetch_all(&self.db)
        .await
        .unwrap()
    }

    pub async fn get(&self, id: u64) -> Option<Transaction> {
        sqlx::query_as!(
            Transaction,
            "SELECT * FROM transactions WHERE id = ?",
            id
        )
        .fetch_one(&self.db)
        .await
        .ok()
    }

    pub async fn create(
        &self,
        data: &CreateTransactionRequest,
        user_id: u64,
    ) -> Option<Transaction> {
        let query_result = sqlx::query!(
            "INSERT INTO transactions (user_id, category_id, type, amount, memo, description) VALUES (?, ?, ?, ?, ?, ?)",
            user_id,
            data.category_id,
            data.r#type,
            data.amount,
            data.memo,
            data.description,
        ).execute(&self.db).await.unwrap();

        self.get(query_result.last_insert_id()).await
    }

    pub async fn update(
        &self,
        transaction: &UpdateTransactionRequest,
        id: u64,
    ) {
        sqlx::query!(
            "UPDATE transactions SET memo = ?, description = ? WHERE id = ?",
            transaction.memo,
            transaction.description,
            id
        )
        .execute(&self.db)
        .await
        .unwrap();
    }

    pub async fn delete(&self, id: u64) {
        sqlx::query!("DELETE FROM transactions  WHERE id = ?", id)
            .execute(&self.db)
            .await
            .unwrap();
    }
}
