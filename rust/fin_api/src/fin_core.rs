use async_trait::async_trait;
use base64::{engine, Engine as _};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use thiserror::Error;

pub struct INGTransaction {
    pub date: String,
    pub description: String,
    pub credit: f64,
    pub debit: f64,
}

pub struct PaginationDetails {
    pub page: Option<i32>,
    pub limit: i32,
}

impl PaginationDetails {
    pub fn new(page: i32, limit: i32) -> Self {
        Self {
            page: Some(page),
            limit,
        }
    }

    pub fn limit(limit: i32) -> Self {
        Self { page: None, limit }
    }

    pub fn offset(&self) -> i32 {
        match self.page {
            Some(page) => (page - 1) * self.limit,
            None => 0,
        }
    }
}

pub fn get_timestamp_start_of_month(timestamp: i64) -> i64 {
    let naive_date_time =
        NaiveDateTime::from_timestamp_opt(timestamp, 0).expect("Invalid timestamp");
    let start_of_month_date =
        NaiveDate::from_ymd_opt(naive_date_time.year(), naive_date_time.month(), 1)
            .expect("Invalid timestamp");
    let start_of_month_datetime = start_of_month_date
        .and_hms_opt(0, 0, 0)
        .expect("Invalid timestamp");

    start_of_month_datetime.timestamp()
}

#[derive(Error, Debug)]
pub enum FinError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Item not found: {0}")]
    NotFound(String),
    #[error("Item already exists: {0}")]
    AlreadyExists(String),
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type_id: String,
    pub amount_cents: i64,
    pub date: i64,
    pub description: String,
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnprocessedTransaction {
    pub id: String,
    pub amount_cents: i64,
    pub date: i64,
    pub description: String,
}

impl PartialOrd for UnprocessedTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for UnprocessedTransaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnproccessedTransactionCreationArgs {
    pub amount_cents: i64,
    pub date: i64,
    pub description: String,
}

impl From<UnproccessedTransactionCreationArgs> for UnprocessedTransaction {
    fn from(args: UnproccessedTransactionCreationArgs) -> Self {
        let string = format!("{}{}{}", args.amount_cents, args.date, args.description);
        let id = engine::general_purpose::STANDARD_NO_PAD.encode(string.as_bytes());
        Self {
            id,
            amount_cents: args.amount_cents,
            date: args.date,
            description: args.description,
        }
    }
}

#[async_trait]
pub trait TransactionRepository {
    async fn get_by_id(&self, id: &str) -> Result<Option<Transaction>, FinError>;
    async fn get_by_month(
        &self,
        month: i64,
        pagintaion: Option<PaginationDetails>,
    ) -> Result<Vec<Transaction>, FinError>;
    async fn create(&self, transaction: Transaction) -> Result<Transaction, FinError>;
}

#[async_trait]
pub trait UnproccessedTransactionRepository {
    async fn get_all(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError>;
    async fn create(
        &self,
        transaction: UnproccessedTransactionCreationArgs,
    ) -> Result<UnprocessedTransaction, FinError>;
    async fn delete(&self, id: &str) -> Result<UnprocessedTransaction, FinError>;
}

#[async_trait]
pub trait TransactionTypeRepository {
    async fn get_all(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<TransactionType>, FinError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<TransactionType>, FinError>;
    async fn create(&self, name: &str) -> Result<TransactionType, FinError>;
    async fn update(&self, transaction_type: TransactionType) -> Result<TransactionType, FinError>;
}

#[async_trait]
pub trait ClassifyingRuleRepository {
    async fn get_all(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<ClassifyingRuleList, FinError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError>;
    async fn create(&self, args: ClassifyingRuleCreationArgs) -> Result<ClassifyingRule, FinError>;
    async fn update(&self, rule: ClassifyingRuleUpdateArgs) -> Result<ClassifyingRule, FinError>;
    async fn delete(&self, id: &str) -> Result<ClassifyingRule, FinError>;
    async fn reorder(&self, id: &str, after: Option<&str>)
        -> Result<ClassifyingRuleList, FinError>;
}

#[async_trait]
pub trait FinApi {
    async fn get_all_transaction_types(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<TransactionType>, FinError>;
    async fn get_transaction_type(&self, id: &str) -> Result<Option<TransactionType>, FinError>;
    async fn create_transaction_type(&self, name: &str) -> Result<TransactionType, FinError>;
    async fn update_transaction_type(
        &self,
        transaction_type: TransactionType,
    ) -> Result<TransactionType, FinError>;

    async fn get_all_rules(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<ClassifyingRuleList, FinError>;
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
    async fn reorder_rule(
        &self,
        id: &str,
        after: Option<&str>,
    ) -> Result<ClassifyingRuleList, FinError>;
    async fn process(&self) -> Result<u32, FinError>;
    async fn get_all_unprocessed_transactions(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError>;
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
        + ClassifyingRuleRepository
        + UnproccessedTransactionRepository
        + TransactionRepository,
{
    async fn get_all_transaction_types(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<TransactionType>, FinError> {
        TransactionTypeRepository::get_all(&self.db, pagination).await
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

    async fn get_all_rules(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<ClassifyingRuleList, FinError> {
        ClassifyingRuleRepository::get_all(&self.db, pagination).await
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

    async fn reorder_rule(
        &self,
        id: &str,
        after: Option<&str>,
    ) -> Result<ClassifyingRuleList, FinError> {
        ClassifyingRuleRepository::reorder(&self.db, id, after).await
    }

    async fn process(&self) -> Result<u32, FinError> {
        let rules = self.get_all_rules(None).await?;
        if rules.len() == 0 {
            return Ok(0);
        }
        let unprocessed_transactions = self.get_all_unprocessed_transactions(None).await?;
        if unprocessed_transactions.len() == 0 {
            return Ok(0);
        }

        let mut created_count: u32 = 0;

        for unprocessed_transaction in &unprocessed_transactions {
            let mut found = false;
            for rule in &rules {
                if unprocessed_transaction.description.contains(&rule.pattern) {
                    let transaction = Transaction {
                        id: unprocessed_transaction.id.clone(),
                        transaction_type_id: rule.transaction_type_id.clone(),
                        amount_cents: unprocessed_transaction.amount_cents,
                        description: unprocessed_transaction.description.clone(),
                        date: unprocessed_transaction.date,
                    };
                    TransactionRepository::create(&self.db, transaction).await?;
                    UnproccessedTransactionRepository::delete(
                        &self.db,
                        &unprocessed_transaction.id,
                    )
                    .await?;
                    created_count += 1;
                    found = true;
                }
            }
            if !found {
                break
            }
        }
        Ok(created_count)
    }

    async fn get_all_unprocessed_transactions(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError> {
        UnproccessedTransactionRepository::get_all(&self.db, pagination).await
    }
}
