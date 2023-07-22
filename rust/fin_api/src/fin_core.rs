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

#[derive(Debug, PartialEq, Clone)]
pub struct ClassifyingRuleUpdateArgs {
    pub id: String,
    pub name: Option<String>,
    pub transaction_type_id: Option<String>,
    pub pattern: Option<String>,
}

impl ClassifyingRuleUpdateArgs {
    pub fn merge_existing(self, rule: ClassifyingRule) -> ClassifyingRule {
        ClassifyingRule {
            id: self.id,
            name: self.name.unwrap_or(rule.name),
            transaction_type_id: self.transaction_type_id.unwrap_or(rule.transaction_type_id),
            pattern: self.pattern.unwrap_or(rule.pattern),
        }
    }
}

impl ClassifyingRuleCreationArgs {
    pub fn to_rule(self, id: String) -> ClassifyingRule {
        ClassifyingRule {
            id,
            name: self.name,
            transaction_type_id: self.transaction_type_id,
            pattern: self.pattern,
        }
    }
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
    async fn update(&self, rule: ClassifyingRuleUpdateArgs) -> Result<ClassifyingRule, FinError>;
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
    async fn update_rule(
        &self,
        rule: ClassifyingRuleUpdateArgs,
    ) -> Result<ClassifyingRule, FinError>;
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
        let transaction_type = self.get_transaction_type(&args.transaction_type_id).await?;
        if transaction_type.is_none() {
            return Err(FinError::NotFound(format!(
                "Transaction type with id {}",
                args.transaction_type_id
            )));
        }
        ClassifyingRuleRepository::create(&self.db, args).await
    }

    async fn update_rule(
        &self,
        rule: ClassifyingRuleUpdateArgs,
    ) -> Result<ClassifyingRule, FinError> {
        if let Some(transacction_type_id) = &rule.transaction_type_id {
            let transaction_type = self.get_transaction_type(transacction_type_id).await?;
            if transaction_type.is_none() {
                return Err(FinError::NotFound(format!(
                    "Transaction type with id {}",
                    transacction_type_id
                )));
            }
        }
        ClassifyingRuleRepository::update(&self.db, rule).await
    }

    async fn delete_rule(&self, id: &str) -> Result<ClassifyingRule, FinError> {
        ClassifyingRuleRepository::delete(&self.db, id).await
    }

    async fn reorder_rule(&self, id: &str, after: &str) -> Result<ClassifyingRuleList, FinError> {
        ClassifyingRuleRepository::reorder(&self.db, id, after).await
    }
}
