use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ---- ENTITIES ----
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionType {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub tansaction_type: TransactionType,
    pub cents: f64,
    pub date: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnclassifiedTransaction {
    pub id: String,
    pub cents: f64,
    pub date: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifyingRule {
    pub id: String,
    pub transaction_type: TransactionType,
    pub description: String,
}

// ---- ERRORS ----

#[derive(Error, Debug)]
pub enum FinError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Item not found: {0}")]
    NotFound(String),
    #[error("Item already exists: {0}")]
    AlreadyExists(String),
    #[error("Incorrect format for type {0}: {1}")]
    InvalidFormat(String, String),
}

// ---- API_TRAIT ----

#[async_trait]
pub trait FinApi {
    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError>;

    async fn get_transaction_type(&self, id: &str) -> Result<Option<TransactionType>, FinError>;

    async fn get_transaction_types<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<TransactionType>>, FinError>
    where
        T: Into<Pagination> + Send + Sync;

    async fn update_transaction_type(
        &self,
        id: &str,
        name: &str,
    ) -> Result<TransactionType, FinError>;

    async fn create_transaction(
        &self,
        data: CreateTransactionData,
    ) -> Result<Transaction, FinError>;

    async fn get_transaction(&self, id: &str) -> Result<Option<Transaction>, FinError>;

    async fn get_transactions<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<Transaction>>, FinError>
    where
        T: Into<Pagination> + Send + Sync;

    async fn update_transaction(
        &self,
        id: &str,
        data: UpdateTransactionData,
    ) -> Result<Transaction, FinError>;

    async fn delete_transaction(&self, id: &str) -> Result<Transaction, FinError>;

    async fn query_transactions_by_date_range<T>(
        &self,
        start_date: i64,
        end_date: i64,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<Transaction>>, FinError>
    where
        T: Into<Pagination> + Send + Sync;

    async fn create_unclassified_transaction(
        &self,
        data: CreateUnclassifiedTransactionData,
    ) -> Result<UnclassifiedTransaction, FinError>;

    async fn get_unclassified_transaction(
        &self,
        id: &str,
    ) -> Result<Option<UnclassifiedTransaction>, FinError>;

    async fn get_unclassified_transactions<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<UnclassifiedTransaction>>, FinError>
    where
        T: Into<Pagination> + Send + Sync;

    async fn update_unclassified_transaction(
        &self,
        id: &str,
        data: UpdateUnclassifiedTransactionData,
    ) -> Result<UnclassifiedTransaction, FinError>;

    async fn delete_unclassified_transaction(
        &self,
        id: &str,
    ) -> Result<UnclassifiedTransaction, FinError>;

    async fn create_classifying_rule(
        &self,
        data: CreateClassifyingRuleData<'_>,
    ) -> Result<ClassifyingRule, FinError>;

    async fn reorder_classifying_rules(
        &self,
        source_id: &str,
        target_id: Option<&str>,
    ) -> Result<u32, FinError>;

    async fn get_classifying_rule(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError>;

    async fn get_classifying_rules<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<ClassifyingRule>>, FinError>
    where
        T: Into<Pagination> + Send + Sync;

    async fn update_classifying_rule(
        &self,
        id: &str,
        data: UpdateClassifyingRuleData,
    ) -> Result<ClassifyingRule, FinError>;

    async fn delete_classifying_rule(&self, id: &str) -> Result<ClassifyingRule, FinError>;

    async fn classify_transactions<T>(
        &self,
        transactions: T,
        stop_condition: ClassifyTransactionStopCondition,
        store_unclassified: bool,
    ) -> Result<TransactionClassificationResult, FinError>
    where
        T: IntoIterator<Item = UnclassifiedTransaction> + Send + Sync;

    async fn classify_stored_transactions(
        &self,
        stop_condition: ClassifyTransactionStopCondition,
    ) -> Result<TransactionClassificationResult, FinError>;

    async fn generate_report(&self, start_date: i64, end_date: i64) -> Result<Report, FinError>;
}

// ---- CONTRACTS ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: T,
    pub pagination: OutputPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub start_date: i64,
    pub end_date: i64,
    pub by_type: HashMap<String, String>,
    pub total: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionClassificationResult {
    pub classified_count: u32,
    pub unclassified_count: u32,
    pub total_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Pagination {
    LimitAndPage { limit: u32, page: u32 },
    None,
}

impl Default for Pagination {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutputPagination {
    None,
}

impl Default for OutputPagination {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClassifyTransactionStopCondition {
    AlreadyExists,
    Limit(u32),
    Date(i64),
    None,
}

impl Default for ClassifyTransactionStopCondition {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClassifiyStoredTransactionStopCondition {
    Limit(u32),
    Date(i64),
    None,
}

impl Default for ClassifiyStoredTransactionStopCondition {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionData<'a> {
    pub transaction_type_id: &'a str,
    pub cents: i64,
    pub date: i64,
    pub description: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionData<'a> {
    pub transaction_type_id: Option<&'a str>,
    pub cents: Option<i64>,
    pub date: Option<i64>,
    pub description: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnclassifiedTransactionData<'a> {
    pub cents: i64,
    pub date: i64,
    pub description: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnclassifiedTransactionData<'a> {
    pub cents: Option<i64>,
    pub date: Option<i64>,
    pub description: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClassifyingRuleData<'a> {
    pub transaction_type_id: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClassifyingRuleData<'a> {
    pub transaction_type_id: Option<&'a str>,
    pub description: Option<&'a str>,
}

// ---- REPOSITORY_TRAITS ----

#[async_trait]
pub trait TransactionTypeRepository {
    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError>;

    async fn get_transaction_type(&self, id: &str) -> Result<Option<TransactionType>, FinError>;

    async fn get_transaction_types<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<TransactionType>>, FinError>
    where
        T: Into<Pagination>;

    async fn update_transaction_type(
        &self,
        id: &str,
        name: &str,
    ) -> Result<TransactionType, FinError>;

    async fn delete_transaction_type(&self, id: &str) -> Result<TransactionType, FinError>;
}

#[async_trait]
pub trait ClassifyingRuleRepository {
    async fn create_classifying_rule(
        &self,
        data: CreateClassifyingRuleData,
    ) -> Result<ClassifyingRule, FinError>;

    async fn get_classifying_rule(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError>;

    async fn get_classifying_rules<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<ClassifyingRule>>, FinError>
    where
        T: Into<Pagination>;

    async fn update_classifying_rule(
        &self,
        id: &str,
        data: UpdateClassifyingRuleData,
    ) -> Result<ClassifyingRule, FinError>;

    async fn delete_classifying_rule(&self, id: &str) -> Result<ClassifyingRule, FinError>;

    async fn reorder_classifying_rules(
        &self,
        source_id: &str,
        target_id: Option<&str>,
    ) -> Result<u32, FinError>;
}

#[async_trait]
pub trait UnclassifiedTransactionRepository {
    async fn create_unclassified_transaction(
        &self,
        data: CreateUnclassifiedTransactionData,
    ) -> Result<UnclassifiedTransaction, FinError>;

    async fn get_unclassified_transaction(
        &self,
        id: &str,
    ) -> Result<Option<UnclassifiedTransaction>, FinError>;

    async fn get_unclassified_transactions<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<UnclassifiedTransaction>>, FinError>
    where
        T: Into<Pagination>;

    async fn update_unclassified_transaction(
        &self,
        id: &str,
        data: UpdateUnclassifiedTransactionData,
    ) -> Result<UnclassifiedTransaction, FinError>;

    async fn delete_unclassified_transaction(
        &self,
        id: &str,
    ) -> Result<UnclassifiedTransaction, FinError>;
}

#[async_trait]
pub trait TransactionRepository {
    async fn create_transaction(
        &self,
        data: CreateTransactionData,
    ) -> Result<Transaction, FinError>;

    async fn get_transaction(&self, id: &str) -> Result<Option<Transaction>, FinError>;

    async fn get_transactions<T>(
        &self,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<Transaction>>, FinError>
    where
        T: Into<Pagination>;

    async fn update_transaction(
        &self,
        id: &str,
        data: UpdateTransactionData,
    ) -> Result<Transaction, FinError>;

    async fn delete_transaction(&self, id: &str) -> Result<Transaction, FinError>;

    async fn query_transactions_by_date_range<T>(
        &self,
        start_date: i64,
        end_date: i64,
        pagination: T,
    ) -> Result<PaginatedResult<Vec<Transaction>>, FinError>
    where
        T: Into<Pagination>;
}

// ---- API_IMPL ----

pub struct Api<T> {
    db: T,
}

impl<T> Api<T> {
    pub fn new(db: T) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<T> FinApi for Api<T>
where
    T: TransactionRepository
        + UnclassifiedTransactionRepository
        + ClassifyingRuleRepository
        + TransactionTypeRepository
        + Send
        + Sync,
{
    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError> {
        todo!()
    }

    async fn get_transaction_type(&self, id: &str) -> Result<Option<TransactionType>, FinError> {
        todo!()
    }

    async fn get_transaction_types<K>(
        &self,
        pagination: K,
    ) -> Result<PaginatedResult<Vec<TransactionType>>, FinError>
    where
        K: Into<Pagination> + Send + Sync,
    {
        todo!()
    }

    async fn update_transaction_type(
        &self,
        id: &str,
        name: &str,
    ) -> Result<TransactionType, FinError> {
        todo!()
    }

    async fn create_transaction(
        &self,
        data: CreateTransactionData,
    ) -> Result<Transaction, FinError> {
        todo!()
    }

    async fn get_transaction(&self, id: &str) -> Result<Option<Transaction>, FinError> {
        todo!()
    }

    async fn get_transactions<K>(
        &self,
        pagination: K,
    ) -> Result<PaginatedResult<Vec<Transaction>>, FinError>
    where
        K: Into<Pagination> + Send + Sync,
    {
        todo!()
    }

    async fn update_transaction(
        &self,
        id: &str,
        data: UpdateTransactionData,
    ) -> Result<Transaction, FinError> {
        todo!()
    }

    async fn delete_transaction(&self, id: &str) -> Result<Transaction, FinError> {
        todo!()
    }

    async fn query_transactions_by_date_range<K>(
        &self,
        start_date: i64,
        end_date: i64,
        pagination: K,
    ) -> Result<PaginatedResult<Vec<Transaction>>, FinError>
    where
        K: Into<Pagination> + Send + Sync,
    {
        todo!()
    }

    async fn create_unclassified_transaction(
        &self,
        data: CreateUnclassifiedTransactionData,
    ) -> Result<UnclassifiedTransaction, FinError> {
        todo!()
    }

    async fn get_unclassified_transaction(
        &self,
        id: &str,
    ) -> Result<Option<UnclassifiedTransaction>, FinError> {
        todo!()
    }

    async fn get_unclassified_transactions<K>(
        &self,
        pagination: K,
    ) -> Result<PaginatedResult<Vec<UnclassifiedTransaction>>, FinError>
    where
        K: Into<Pagination> + Send + Sync,
    {
        todo!()
    }

    async fn update_unclassified_transaction(
        &self,
        id: &str,
        data: UpdateUnclassifiedTransactionData,
    ) -> Result<UnclassifiedTransaction, FinError> {
        todo!()
    }

    async fn delete_unclassified_transaction(
        &self,
        id: &str,
    ) -> Result<UnclassifiedTransaction, FinError> {
        todo!()
    }

    async fn create_classifying_rule(
        &self,
        data: CreateClassifyingRuleData<'_>,
    ) -> Result<ClassifyingRule, FinError> {
        todo!()
    }

    async fn reorder_classifying_rules(
        &self,
        source_id: &str,
        target_id: Option<&str>,
    ) -> Result<u32, FinError> {
        todo!()
    }

    async fn get_classifying_rule(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError> {
        todo!()
    }

    async fn get_classifying_rules<K>(
        &self,
        pagination: K,
    ) -> Result<PaginatedResult<Vec<ClassifyingRule>>, FinError>
    where
        K: Into<Pagination> + Send + Sync,
    {
        todo!()
    }

    async fn update_classifying_rule(
        &self,
        id: &str,
        data: UpdateClassifyingRuleData,
    ) -> Result<ClassifyingRule, FinError> {
        todo!()
    }

    async fn delete_classifying_rule(&self, id: &str) -> Result<ClassifyingRule, FinError> {
        todo!()
    }

    async fn classify_transactions<K>(
        &self,
        transactions: K,
        stop_condition: ClassifyTransactionStopCondition,
        store_unclassified: bool,
    ) -> Result<TransactionClassificationResult, FinError>
    where
        K: IntoIterator<Item = UnclassifiedTransaction> + Send + Sync,
    {
        todo!()
    }

    async fn classify_stored_transactions(
        &self,
        stop_condition: ClassifyTransactionStopCondition,
    ) -> Result<TransactionClassificationResult, FinError>
    {
        todo!()
    }

    async fn generate_report(&self, start_date: i64, end_date: i64) -> Result<Report, FinError> {
        todo!()
    }
}
