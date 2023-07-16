use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FinError {
    #[error("Database error: {0}")]
    DbError(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct TransactionType {
    pub id: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassifyingRule {
    pub id: String,
    pub name: String,
    pub transaction_type_id: String,
    pub pattern: String,
}

pub type ClassifyingRuleList = Vec<ClassifyingRule>;

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type_id: String,
    pub ammount_cents: i64,
    pub date: i64,
    pub description: String,
}

#[async_trait]
pub trait TransactionTypeRepository {
    async fn get_all(&self) -> Result<Vec<TransactionType>, FinError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<TransactionType>, FinError>;
    async fn create(&self, name: &str) -> Result<TransactionType, FinError>;
    async fn update(&self, id: &str, name: &str) -> Result<TransactionType, FinError>;
}

pub struct FinApi<Db: TransactionTypeRepository> {
    db: Db,
}

impl<Db: TransactionTypeRepository> FinApi<Db> {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
    pub async fn get_all_transaction_types(&self) -> Result<Vec<TransactionType>, FinError> {
        self.db.get_all().await
    }

    pub async fn get_transaction_type(
        &self,
        id: &str,
    ) -> Result<Option<TransactionType>, FinError> {
        self.db.get_by_id(id).await
    }

    pub async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError> {
        self.db.create(name).await
    }

    pub async fn update_transaction_type(
        &self,
        id: &str,
        name: &str,
    ) -> Result<TransactionType, FinError> {
        self.db.update(id, name).await
    }
}
