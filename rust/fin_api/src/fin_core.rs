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
    async fn create_transaction_type(
        &self,
        input: CreateTransactionTypeInput,
    ) -> Result<CreateTransactionTypeOutput, FinError>;

    async fn get_transaction_types(
        &self,
        input: GetTransactionTypesInput,
    ) -> Result<GetTransactionTypesOutput, FinError>;

    async fn get_transaction_type(
        &self,
        input: GetTransactionTypeInput,
    ) -> Result<GetTransactionTypeOutput, FinError>;

    async fn update_transaction_type(
        &self,
        input: UpdateTransactionTypeInput,
    ) -> Result<UpdateTransactionTypeOutput, FinError>;

    async fn create_rule(
        &self,
        input: CreateClassifyingRuleInput,
    ) -> Result<CreateClassifyingRuleOutput, FinError>;

    async fn get_classifying_rules(
        &self,
        input: GetClassifyingRulesInput,
    ) -> Result<GetClassifyingRulesOutput, FinError>;

    async fn get_rule(
        &self,
        input: GetClassifyingRuleInput,
    ) -> Result<GetClassifyingRuleOutput, FinError>;

    async fn update_rule(
        &self,
        input: UpdateClassifyingRuleInput,
    ) -> Result<UpdateTransactionTypeOutput, FinError>;

    async fn delete_rule(
        &self,
        input: DeleteClassifyingRuleInput,
    ) -> Result<DeleteClassifyingRuleOutput, FinError>;

    async fn created_unclassified_transaction(
        &self,
        input: CreateUnclassifiedTransactionInput,
    );

    async fn get_unclassified_transactions(
        &self,
        input: GetUnclassifiedTransactionsInput,
    ) -> Result<GetUnclassifiedTransactionsOutput, FinError>;

    async fn get_unclassified_transaction(
        &self,
        input: GetUnclassifiedTransactionInput,
    ) -> Result<GetUnclassifiedTransactionOutput, FinError>;

    async fn update_unclassified_transaction(
        &self,
        input: UpdateUnclassifiedTransactionInput,
    ) -> Result<UpdateUnclassifiedTransactionOutput, FinError>;

    async fn delete_unclassified_transaction(
        &self,
        input: DeleteUnclassifiedTransactionInput,
    ) -> Result<DeleteUnclassifiedTransactionOutput, FinError>;

    async fn create_transaction(
        &self,
        input: CreateTransactionInput,
    ) -> Result<CreateTransactionOutput, FinError>;

    async fn get_transactions(
        &self,
        input: GetTransactionsInput,
    ) -> Result<GetTransactionsOutput, FinError>;

    async fn get_transaction(
        &self,
        input: GetTransactionInput,
    ) -> Result<GetTransactionOutput, FinError>;

    async fn update_transaction(
        &self,
        input: UpdateTransactionInput,
    ) -> Result<UpdateTransactionOutput, FinError>;

    async fn delete_transaction(
        &self,
        input: DeleteTransactionInput,
    ) -> Result<DeleteTransactionOutput, FinError>;
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
pub struct GetTransactionTypesInput {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypesOutput {
    pub transaction_types: Vec<TransactionType>,
    pub pagination: OutputPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypeInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionTypeOutput {
    pub transaction_type: Option<TransactionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionTypeInput {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionTypeOutput {
    pub created_transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionTypeInput {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionTypeOutput {
    pub updated_transaction_type: TransactionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRulesInput {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRuleInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRuleOutput {
    pub classifying_rule: Option<ClassifyingRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClassifyingRulesOutput {
    pub classifying_rules: Vec<ClassifyingRule>,
    pub pagination: OutputPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClassifyingRuleInput {
    #[serde(default)]
    pub description: String,
    pub pattern: String,
    pub name: String,
    pub transaction_type_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateClassifyingRuleOutput {
    pub created_classifying_rule: ClassifyingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateClassifyingRuleInput {
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
pub struct UpdateClassifyingRuleOutput {
    pub updated_classifying_rule: ClassifyingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteClassifyingRuleInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteClassifyingRuleOutput {
    pub deleted_classifying_rule: ClassifyingRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderClassifyingRulesInput {
    pub source_id: String,
    pub after_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderClassifyingRulesOutput {
    pub new_position_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnclassifiedTransactionsInput {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnclassifiedTransactionsOutput {
    pub unclassified_transactions: Vec<UnclassifiedTransaction>,
    pub pagination: OutputPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnclassifiedTransactionInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUnclassifiedTransactionOutput {
    pub unclassified_transaction: Option<UnclassifiedTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnclassifiedTransactionInput {
    pub cents: i64,
    pub description: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUnclassifiedTransactionOutput {
    pub created_unclassified_transaction: UnclassifiedTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUnclassifiedTransactionInput {
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
pub struct UpdateUnclassifiedTransactionOutput {
    pub updated_unclassified_transaction: UnclassifiedTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnclassifiedTransactionInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnclassifiedTransactionOutput {
    pub deleted_unclassified_transaction: UnclassifiedTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionInput {
    pub cents: i64,
    pub description: String,
    pub date: String,
    pub transaction_type_id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionOutput {
    pub created_transaction: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionsInput {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionsOutput {
    pub transactions: Vec<Transaction>,
    pub pagination: OutputPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionOutput {
    pub transaction: Option<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionInput {
    pub id: String,
    pub cents: Option<i64>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub transaction_type_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTransactionOutput {
    pub updated_transaction: Transaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTransactionInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTransactionOutput {
    pub deleted_transaction: Transaction,
}

