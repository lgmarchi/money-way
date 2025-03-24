use crate::domain::transaction::Transaction;

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
}
