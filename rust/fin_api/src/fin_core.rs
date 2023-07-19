use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FinError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Item not found: {0}")]
    NotFound(String),
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

#[derive(Debug, PartialEq, Clone)]
pub struct ClassifyingRuleCreationArgs {
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
pub trait ClassifyingRuleRepository {
    async fn get_all(&self) -> Result<ClassifyingRuleList, FinError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError>;
    async fn create(&self, args: ClassifyingRuleCreationArgs) -> Result<ClassifyingRule, FinError>;
    async fn update(&self, rule: ClassifyingRule) -> Result<ClassifyingRule, FinError>;
    async fn delete(&self, id: &str) -> Result<ClassifyingRule, FinError>;
    async fn reorder(&self, id: &str, after: &str) -> Result<ClassifyingRuleList, FinError>;
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

    async fn get_all_rules(&self) -> Result<ClassifyingRuleList, FinError>;
    async fn get_rule(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError>;
    async fn create_rule(
        &self,
        args: ClassifyingRuleCreationArgs,
    ) -> Result<ClassifyingRule, FinError>;
    async fn update_rule(&self, rule: ClassifyingRule) -> Result<ClassifyingRule, FinError>;
    async fn delete_rule(&self, id: &str) -> Result<ClassifyingRule, FinError>;
    async fn reorder_rule(&self, id: &str, after: &str) -> Result<ClassifyingRuleList, FinError>;
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
impl<Db> FinApi for FinApiService<Db>
where
    Db: TransactionTypeRepository
        + std::marker::Send
        + std::marker::Sync
        + ClassifyingRuleRepository,
{
    async fn get_all_transaction_types(&self) -> Result<Vec<TransactionType>, FinError> {
        TransactionTypeRepository::get_all(&self.db).await
    }

    async fn get_transaction_type(&self, id: &str) -> Result<Option<TransactionType>, FinError> {
        TransactionTypeRepository::get_by_id(&self.db, id).await
    }

    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError> {
        TransactionTypeRepository::create(&self.db, name).await
    }

    async fn update_transaction_type(
        &self,
        transaction_type: TransactionType,
    ) -> Result<TransactionType, FinError> {
        TransactionTypeRepository::update(&self.db, transaction_type).await
    }

    async fn get_all_rules(&self) -> Result<ClassifyingRuleList, FinError> {
        ClassifyingRuleRepository::get_all(&self.db).await
    }

    async fn get_rule(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError> {
        ClassifyingRuleRepository::get_by_id(&self.db, id).await
    }

    async fn create_rule(
        &self,
        args: ClassifyingRuleCreationArgs,
    ) -> Result<ClassifyingRule, FinError> {
        ClassifyingRuleRepository::create(&self.db, args).await
    }

    async fn update_rule(&self, rule: ClassifyingRule) -> Result<ClassifyingRule, FinError> {
        ClassifyingRuleRepository::update(&self.db, rule).await
    }

    async fn delete_rule(&self, id: &str) -> Result<ClassifyingRule, FinError> {
        ClassifyingRuleRepository::delete(&self.db, id).await
    }

    async fn reorder_rule(&self, id: &str, after: &str) -> Result<ClassifyingRuleList, FinError> {
        ClassifyingRuleRepository::reorder(&self.db, id, after).await
    }
}
