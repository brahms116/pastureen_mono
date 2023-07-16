use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FinError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Item not found: {0}")]
    NotFound(String)
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
    pub amount_cents: i64,
    pub date: i64,
    pub description: String,
}

#[async_trait]
pub trait TransactionTypeRepository {
    async fn get_all(&self) -> Result<Vec<TransactionType>, FinError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<TransactionType>, FinError>;
    async fn create(&self, name: &str) -> Result<TransactionType, FinError>;
    async fn update(&self, transaction_type: TransactionType) -> Result<TransactionType, FinError>;
}

#[async_trait]
pub trait FinApi {
    async fn get_all_transaction_types(&self) -> Result<Vec<TransactionType>, FinError>;
    async fn get_transaction_type(&self, id: &str) -> Result<Option<TransactionType>, FinError>;
    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError>;
    async fn update_transaction_type(
        &self,
        transaction_type: TransactionType,
    ) -> Result<TransactionType, FinError>;
}

pub struct FinApiService<Db> {
    db: Db,
}
impl<Db> FinApiService<Db> {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<Db: TransactionTypeRepository + std::marker::Send + std::marker::Sync> FinApi for FinApiService<Db> {
    async fn get_all_transaction_types(&self) -> Result<Vec<TransactionType>, FinError> {
        self.db.get_all().await
    }

    async fn get_transaction_type(
        &self,
        id: &str,
    ) -> Result<Option<TransactionType>, FinError> {
        self.db.get_by_id(id).await
    }

    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError> {
        self.db.create(name).await
    }

    async fn update_transaction_type(
        &self,
        transaction_type: TransactionType,
    ) -> Result<TransactionType, FinError> {
        self.db.update(transaction_type).await
    }
}
