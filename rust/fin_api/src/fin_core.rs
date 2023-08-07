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
pub struct UnprocessedTransaction {
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

// ---- CONTRACTS ----

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
pub enum ResponsePagination {
    None,
}

impl Default for ResponsePagination {
    fn default() -> Self {
        Self::None
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypesRequest {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypesResponse {
    pub transaction_types: Vec<TransactionType>,
    pub pagination: ResponsePagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypeRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypeResponse {
    pub transaction_type: Option<TransactionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionTypeRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionTypeResponse {
    pub created_transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionTypeRequest {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionTypeResponse {
    pub updated_transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRulesRequest {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRuleRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRuleResponse {
    pub classifying_rule: Option<ClassifyingRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRulesResponse {
    pub classifying_rules: Vec<ClassifyingRule>,
    pub pagination: ResponsePagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClassifyingRuleRequest {
    #[serde(default)]
    pub description: String,
    pub pattern: String,
    pub name: String,
    pub transaction_type_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClassifyingRuleResponse {
    pub created_classifying_rule: ClassifyingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClassifyingRuleRequest {
    pub id: String,
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub pattern: Option<String>,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub transaction_type_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClassifyingRuleResponse {
    pub updated_classifying_rule: ClassifyingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteClassifyingRuleRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteClassifyingRuleResponse {
    pub deleted_classifying_rule: ClassifyingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderClassifyingRulesRequest {
    pub source_id: String,
    pub after_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderClassifyingRulesResponse {
    pub new_position_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnprocessedTransactionsRequest {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnprocessedTransactionsResponse {
    pub unprocessed_transactions: Vec<UnprocessedTransaction>,
    pub pagination: ResponsePagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnprocessedTransactionRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnprocessedTransactionResponse {
    pub unprocessed_transaction: Option<UnprocessedTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnprocessedTransactionRequest {
    pub cents: i64,
    pub description: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnprocessedTransactionResponse {
    pub created_unprocessed_transaction: UnprocessedTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnprocessedTransactionRequest {
    pub id: String,
    #[serde(default)]
    pub cents: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnprocessedTransactionResponse {
    pub updated_unprocessed_transaction: UnprocessedTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnprocessedTransactionRequest {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnprocessedTransactionResponse {
    pub deleted_unprocessed_transaction: UnprocessedTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag="type", rename_all = "camelCase")]
pub enum ClassificationStopCondition {
    None,
    DuplicateTransaction,
    FirstFailMatch
}

impl Default for ClassificationStopCondition {
    fn default() -> Self {
        ClassificationStopCondition::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifyStoredTransactionsRequest {
    #[serde(default)]
    pub max_count: Option<u32>,

    #[serde(default)]
    pub stop_condition: ClassificationStopCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifyStoredTransactionsResponse {
    pub classification_result: TransactionClassificationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifyTransactionsRequest {
    pub unprocessed_transactions: Vec<UnprocessedTransaction>,
    #[serde(default)]
    pub stop_condition: ClassificationStopCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifyTransactionsResponse {
    pub classification_result: TransactionClassificationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionsRequest {
    #[serde(default)]
    pub pagination: Pagination,
}

// ---- API_TRAIT ----

#[async_trait]
pub trait FinApi {
    async fn get_transaction_types(
        &self,
        request: GetTransactionTypesRequest,
    ) -> Result<GetTransactionTypesResponse, FinError>;

    async fn get_transaction_type(
        &self,
        request: GetTransactionTypeRequest,
    ) -> Result<GetTransactionTypeResponse, FinError>;

    async fn create_transaction_type(
        &self,
        request: CreateTransactionTypeRequest,
    ) -> Result<CreateTransactionTypeResponse, FinError>;

    async fn update_transaction_type(
        &self,
        request: UpdateTransactionTypeRequest,
    ) -> Result<UpdateTransactionTypeResponse, FinError>;

    async fn get_classifying_rules(
        &self,
        request: GetClassifyingRulesRequest,
    ) -> Result<GetClassifyingRulesResponse, FinError>;

    async fn get_rule(
        &self,
        request: GetClassifyingRuleRequest,
    ) -> Result<GetClassifyingRuleResponse, FinError>;

    async fn create_rule(
        &self,
        request: CreateClassifyingRuleRequest,
    ) -> Result<CreateClassifyingRuleResponse, FinError>;

    async fn update_rule(
        &self,
        request: UpdateClassifyingRuleRequest,
    ) -> Result<UpdateTransactionTypeResponse, FinError>;

    async fn delete_rule(
        &self,
        request: DeleteClassifyingRuleRequest,
    ) -> Result<DeleteClassifyingRuleResponse, FinError>;

    async fn reorder_rule(
        &self,
        id: &str,
        after: Option<&str>,
    ) -> Result<ReorderClassifyingRulesResponse, FinError>;

    async fn process(&self) -> Result<u32, FinError>;

    async fn get_all_unprocessed_transactions(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError>;

    async fn process_ing_transactions(
        &self,
        transactions: &[INGTransaction],
    ) -> Result<ProcessTransactionsResult, FinError>;

    async fn list_transactions(
        &self,
        start_date: i64,
        end_date: i64,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<Transaction>, FinError>;

    async fn generate_report(&self, start_date: i64, end_date: i64) -> Result<Report, FinError>;
}
