use async_trait::async_trait;
use base64::{engine, Engine as _};
use chrono::{Datelike, Months, NaiveDate, NaiveDateTime};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};
use thiserror::Error;

fn format_cents(amount_cents: i64) -> String {
    let amount = amount_cents as f64 / 100.0;
    format!("{:.2}", amount)
}

#[derive(Debug)]
pub struct ProcessTransactionsResult {
    pub processed: u32,
    pub successfully_classified: u32,
}

impl Display for ProcessTransactionsResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Processed {} transactions, {} successfully classified",
            self.processed, self.successfully_classified
        )
    }
}

pub struct Report {
    pub start_date: i64,
    pub end_date: i64,
    pub by_type: HashMap<String, String>,
    pub total: String,
}

#[derive(Debug, Deserialize)]
pub struct INGTransaction {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(default, rename = "Credit")]
    pub credit: Option<f64>,
    #[serde(default, rename = "Debit")]
    pub debit: Option<f64>,
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
    #[error("Incorrect format for type {0}: {1}")]
    InvalidFormat(String, String),
    #[error(
        "Transaction with description {1} has already been processed. Processed {0} transactions"
    )]
    TransactionAlreadyProcessed(ProcessTransactionsResult, String),
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

impl UnprocessedTransaction {
    pub fn new(amount_cents: i64, date: i64, description: String) -> Self {
        let string = format!("{}{}{}", amount_cents, date, description);
        let id = engine::general_purpose::STANDARD_NO_PAD.encode(string.as_bytes());
        Self {
            id,
            amount_cents,
            date,
            description,
        }
    }
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
pub trait UnprocessedTransactionRepository {
    async fn get_all(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError>;
    async fn create(
        &self,
        transaction: UnprocessedTransaction,
    ) -> Result<UnprocessedTransaction, FinError>;
    async fn delete(&self, id: &str) -> Result<UnprocessedTransaction, FinError>;

    async fn get_by_id(&self, id: &str) -> Result<Option<UnprocessedTransaction>, FinError>;
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

pub struct FinApiService<Db> {
    db: Db,
}
impl<Db> FinApiService<Db> {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

fn classify_description<'a>(
    description: &str,
    rules: &'a [ClassifyingRule],
) -> Option<&'a ClassifyingRule> {
    for rule in rules {
        if description.contains(&rule.pattern) {
            return Some(rule);
        }
    }
    None
}

fn classify_transaction<'a>(
    transaction: &UnprocessedTransaction,
    rules: &'a [ClassifyingRule],
) -> Option<&'a ClassifyingRule> {
    classify_description(&transaction.description, rules)
}

impl<Db> FinApiService<Db>
where
    Db: TransactionTypeRepository
        + std::marker::Send
        + std::marker::Sync
        + ClassifyingRuleRepository
        + UnprocessedTransactionRepository
        + TransactionRepository,
{
    async fn process_transaction_with_rules(
        &self,
        unprocessed_transaction: &UnprocessedTransaction,
        rules: &[ClassifyingRule],
    ) -> Result<Option<Transaction>, FinError> {
        let mut rule = classify_transaction(unprocessed_transaction, rules);
        if let Some(rule) = rule.take() {
            let transaction = Transaction {
                id: unprocessed_transaction.id.clone(),
                transaction_type_id: rule.transaction_type_id.clone(),
                amount_cents: unprocessed_transaction.amount_cents,
                description: unprocessed_transaction.description.clone(),
                date: unprocessed_transaction.date,
            };
            let transaction = TransactionRepository::create(&self.db, transaction).await?;
            let existing =
                UnprocessedTransactionRepository::get_by_id(&self.db, &unprocessed_transaction.id)
                    .await?;
            if let Some(_) = existing {
                UnprocessedTransactionRepository::delete(&self.db, &unprocessed_transaction.id).await?;
            }
            return Ok(Some(transaction));
        }
        Ok(None)
    }
}

#[async_trait]
impl<Db> FinApi for FinApiService<Db>
where
    Db: TransactionTypeRepository
        + std::marker::Send
        + std::marker::Sync
        + ClassifyingRuleRepository
        + UnprocessedTransactionRepository
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
            let result = self
                .process_transaction_with_rules(unprocessed_transaction, &rules)
                .await?;
            if let Some(_) = result {
                created_count += 1;
            }
        }
        Ok(created_count)
    }

    async fn get_all_unprocessed_transactions(
        &self,
        pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError> {
        UnprocessedTransactionRepository::get_all(&self.db, pagination).await
    }

    async fn process_ing_transactions(
        &self,
        transactions: &[INGTransaction],
    ) -> Result<ProcessTransactionsResult, FinError> {
        let mut processed_count: u32 = 0;
        let mut classified_count: u32 = 0;
        for transaction in transactions {
            let total_amount = transaction.credit.unwrap_or(0.0) + transaction.debit.unwrap_or(0.0);
            let amount_cents: i64 = (total_amount * 100.0).trunc() as i64;

            // convert date to chrono
            let date = NaiveDate::parse_from_str(&transaction.date, "%e/%m/%Y").map_err(|_| {
                FinError::InvalidFormat(
                    "INGTransaction.date".to_string(),
                    transaction.date.to_string(),
                )
            })?;

            let date = date.and_hms_opt(0, 0, 0).expect("Should be valid time");
            let ts = date.timestamp();

            let unprocessed_transaction =
                UnprocessedTransaction::new(amount_cents, ts, transaction.description.clone());

            let possible_transaction =
                UnprocessedTransactionRepository::get_by_id(&self.db, &unprocessed_transaction.id)
                    .await?;

            if possible_transaction.is_some() {
                // return Err(FinError::TransactionAlreadyProcessed(
                //     ProcessTransactionsResult {
                //         processed: processed_count,
                //         successfully_classified: classified_count,
                //     },
                //     unprocessed_transaction.description.to_string(),
                // ));
                continue;
            }

            let rules = ClassifyingRuleRepository::get_all(&self.db, None).await?;
            // if it passes create a transaction
            let processed_transaction = self
                .process_transaction_with_rules(&unprocessed_transaction, &rules)
                .await?;

            if processed_transaction.is_some() {
                classified_count += 1;
            } else {
                // create an unprocessed transaction
                UnprocessedTransactionRepository::create(&self.db, unprocessed_transaction).await?;
            }
            processed_count += 1;
        }
        Ok(ProcessTransactionsResult {
            processed: processed_count,
            successfully_classified: classified_count,
        })
    }

    async fn list_transactions(
        &self,
        start_date: i64,
        end_date: i64,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<Transaction>, FinError> {
        let mut result = Vec::new();
        if start_date >= end_date {
            return Ok(result);
        }
        let start_datetime = NaiveDateTime::from_timestamp_opt(start_date, 0);
        let end_datetime = NaiveDateTime::from_timestamp_opt(end_date, 0);

        if start_datetime.is_none() || end_datetime.is_none() {
            return Ok(result);
        }
        let start_datetime = start_datetime.expect("should handle invalid");
        let end_datetime = end_datetime.expect("should handle invalid");

        let start_month = NaiveDate::from_ymd_opt(start_datetime.year(), start_datetime.month(), 1)
            .expect("should be valid date")
            .and_hms_opt(0, 0, 0)
            .expect("should be valid time");

        let end_month = NaiveDate::from_ymd_opt(end_datetime.year(), end_datetime.month(), 1)
            .expect("should be valid date")
            .and_hms_opt(0, 0, 0)
            .expect("should be valid time");

        let retrieved_transactions =
            TransactionRepository::get_by_month(&self.db, end_month.timestamp(), None).await?;

        if start_month == end_month {
            result.extend(retrieved_transactions.into_iter().filter(|t| {
                t.date >= start_datetime.timestamp() && t.date <= end_datetime.timestamp()
            }));
            return Ok(result);
        }

        result.extend(
            retrieved_transactions
                .into_iter()
                .filter(|t| t.date <= end_datetime.timestamp()),
        );

        let mut current_month = end_month - Months::new(1);
        while current_month > start_month {
            let retrieved_transactions =
                TransactionRepository::get_by_month(&self.db, current_month.timestamp(), None)
                    .await?;
            result.extend(retrieved_transactions.into_iter());
            current_month = current_month - Months::new(1);
        }
        let retrieved_transactions =
            TransactionRepository::get_by_month(&self.db, start_month.timestamp(), None).await?;

        result.extend(
            retrieved_transactions
                .into_iter()
                .filter(|t| t.date >= start_datetime.timestamp()),
        );
        return Ok(result);
    }

    async fn generate_report(&self, start_date: i64, end_date: i64) -> Result<Report, FinError> {
        let transactions = self.list_transactions(start_date, end_date, None).await?;

        let mut by_type_total = HashMap::<String, i64>::new();
        let mut total: i64 = 0;

        let types = TransactionTypeRepository::get_all(&self.db, None).await?;
        let types = types
            .into_iter()
            .map(|t| (t.id, t.name))
            .collect::<HashMap<String, String>>();

        for transaction in transactions {
            let amount = transaction.amount_cents;
            let transaction_type = transaction.transaction_type_id;
            let transaction_type_name = types.get(&transaction_type).unwrap();
            by_type_total
                .entry(transaction_type_name.to_string())
                .and_modify(|e| *e += amount)
                .or_insert(amount);
            total += amount;
        }

        let by_type: HashMap<String, String> = by_type_total
            .into_iter()
            .map(|(k, v)| (k, format_cents(v)))
            .collect();

        Ok(Report {
            total: format_cents(total),
            start_date,
            end_date,
            by_type,
        })
    }
}
