use super::*;
use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::error::Error;
use tokio_stream::StreamExt;
use uuid::Uuid;

fn get_string_key(key: &str, map: HashMap<String, AttributeValue>) -> Result<String, FinError> {
    let value = map.get(key).ok_or(FinError::DbError(format!(
        "Could not find field {} in dynamodb object",
        key
    )))?;
    let value = value.as_s().map_err(|_| {
        FinError::DbError(format!(
            "Could not convert {} to string in dynamodb object",
            key
        ))
    })?;
    Ok(value.clone())
}

fn get_number_key(key: &str, map: HashMap<String, AttributeValue>) -> Result<String, FinError> {
    let value = map.get(key).ok_or(FinError::DbError(format!(
        "Could not find field {} in dynamodb object",
        key
    )))?;
    let value = value.as_n().map_err(|_| {
        FinError::DbError(format!(
            "Could not convert {} to number in dynamodb object",
            key
        ))
    })?;
    Ok(value.clone())
}

fn get_list_key(
    key: &str,
    map: HashMap<String, AttributeValue>,
) -> Result<Vec<AttributeValue>, FinError> {
    let value = map.get(key).ok_or(FinError::DbError(format!(
        "Could not find field {} in dynamodb  object",
        key
    )))?;
    let value = value.as_l().map_err(|_| {
        FinError::DbError(format!(
            "Could not convert {} to string in dynamodb object",
            key
        ))
    })?;
    Ok(value.clone())
}

impl<E: Error> From<SdkError<E>> for FinError {
    fn from(err: SdkError<E>) -> Self {
        Self::DbError(format!("{:?}", err))
    }
}

impl From<Transaction> for HashMap<String, AttributeValue> {
    fn from(item: Transaction) -> Self {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(item.id.to_string()));
        map.insert(
            "transactionTypeId".to_string(),
            AttributeValue::S(item.transaction_type_id.to_string()),
        );
        map.insert(
            "amountCents".to_string(),
            AttributeValue::N(item.amount_cents.to_string()),
        );

        map.insert(
            "description".to_string(),
            AttributeValue::S(item.description.to_string()),
        );
        map.insert("date".to_string(), AttributeValue::N(item.date.to_string()));
        let month = get_timestamp_start_of_month(item.date);
        map.insert("month".to_string(), AttributeValue::N(month.to_string()));
        map
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Transaction {
    type Error = FinError;
    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = get_string_key("id", value.clone())?;
        let transaction_type_id = get_string_key("transactionTypeId", value.clone())?;

        let amount_cents = get_number_key("amountCents", value.clone())?
            .parse::<i64>()
            .map_err(|e| FinError::DbError(format!("Could not parse amountCents: {}", e)))?;
        let description = get_string_key("description", value.clone())?;

        let date = get_string_key("date", value.clone())?
            .parse::<i64>()
            .map_err(|e| FinError::DbError(format!("Could not parse date: {}", e)))?;

        Ok(Self {
            id,
            transaction_type_id,
            amount_cents,
            description,
            date,
        })
    }
}

impl From<UnprocessedTransaction> for HashMap<String, AttributeValue> {
    fn from(item: UnprocessedTransaction) -> Self {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(item.id.to_string()));
        map.insert(
            "amountCents".to_string(),
            AttributeValue::N(item.amount_cents.to_string()),
        );
        map.insert(
            "description".to_string(),
            AttributeValue::S(item.description.to_string()),
        );
        map.insert("date".to_string(), AttributeValue::N(item.date.to_string()));
        map
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for UnprocessedTransaction {
    type Error = FinError;
    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = get_string_key("id", value.clone())?;
        let amount_cents = get_number_key("amountCents", value.clone())?;
        let amount_cents = amount_cents
            .parse::<i64>()
            .map_err(|_| FinError::DbError("Could not parse amountCents".to_string()))?;

        let description = get_string_key("description", value.clone())?;
        let date = get_string_key("date", value.clone())?;
        let date = date
            .parse::<i64>()
            .map_err(|_| FinError::DbError("Could not parse date".to_string()))?;

        Ok(UnprocessedTransaction {
            id,
            amount_cents,
            description,
            date,
        })
    }
}

impl From<TransactionType> for HashMap<String, AttributeValue> {
    fn from(transaction_type: TransactionType) -> Self {
        let mut map = HashMap::new();
        map.insert(
            "id".to_string(),
            AttributeValue::S(transaction_type.id.to_string()),
        );
        map.insert(
            "name".to_string(),
            AttributeValue::S(transaction_type.name.to_string()),
        );
        map
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for TransactionType {
    type Error = FinError;
    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = value.get("id").ok_or(FinError::DbError(
            "Could not find id in TransactionType".to_string(),
        ))?;
        let id = id
            .as_s()
            .map_err(|_| {
                FinError::DbError("Could not convert id to string in TransactionType".to_string())
            })?
            .clone();

        let name = value.get("name").ok_or(FinError::DbError(
            "Could not find name in TransactionType".to_string(),
        ))?;
        let name = name
            .as_s()
            .map_err(|_| {
                FinError::DbError("Could not convert id to name in TransactionType".to_string())
            })?
            .clone();
        Ok(TransactionType { id, name })
    }
}

pub struct FinDynamoDb {
    pub client: Client,
    pub transaction_type_tablename: String,
    pub classifying_rules_tablename: String,
    pub unprocessed_transactions_tablename: String,
    pub transactions_tablename: String,
}

#[async_trait]
impl TransactionTypeRepository for FinDynamoDb {
    async fn get_all(
        &self,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<TransactionType>, FinError> {
        let scan_builder = self.client.scan();
        let result: Result<Vec<HashMap<String, AttributeValue>>, _> = scan_builder
            .table_name(&self.transaction_type_tablename)
            .into_paginator()
            .items()
            .send()
            .collect()
            .await;

        let result = result?;
        result.into_iter().map(|e| e.try_into()).collect()
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<TransactionType>, FinError> {
        let get_builder = self.client.get_item();
        let result = get_builder
            .table_name(&self.transaction_type_tablename)
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await?;

        let attributes = result.item();

        if let Some(attributes) = attributes {
            return Ok(Some(attributes.clone().try_into()?));
        }
        Ok(None)
    }

    async fn create(&self, name: &str) -> Result<TransactionType, FinError> {
        let id = Uuid::new_v4().to_string();
        let transaction_type = TransactionType {
            id,
            name: name.to_string(),
        };

        let put_builder = self.client.put_item();
        put_builder
            .table_name(&self.transaction_type_tablename)
            .set_item(Some(transaction_type.clone().into()))
            .send()
            .await?;

        Ok(transaction_type)
    }

    async fn update(&self, transaction_type: TransactionType) -> Result<TransactionType, FinError> {
        let put_builder = self.client.put_item();
        let result = put_builder
            .table_name(&self.transaction_type_tablename)
            .set_item(Some(transaction_type.clone().into()))
            .condition_expression("attribute_exists(id)")
            .return_values(ReturnValue::AllOld)
            .send()
            .await;

        if let Err(SdkError::ServiceError(put_item_err)) = result {
            let err = put_item_err.err();
            if let PutItemError::ConditionalCheckFailedException(_) = err {
                return Err(FinError::NotFound(format!(
                    "TransactionType with id {}",
                    transaction_type.id
                )));
            }
        }
        Ok(transaction_type)
    }
}

impl From<ClassifyingRule> for HashMap<String, AttributeValue> {
    fn from(rule: ClassifyingRule) -> Self {
        let mut map = HashMap::new();
        map.insert("id".to_string(), AttributeValue::S(rule.id.to_string()));
        map.insert("name".to_string(), AttributeValue::S(rule.name.to_string()));
        map.insert(
            "transactionTypeId".to_string(),
            AttributeValue::S(rule.transaction_type_id.to_string()),
        );
        map.insert(
            "pattern".to_string(),
            AttributeValue::S(rule.pattern.to_string()),
        );
        map
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for ClassifyingRule {
    type Error = FinError;

    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = get_string_key("id", value.clone())?;
        let name = get_string_key("name", value.clone())?;
        let transaction_type_id = get_string_key("transactionTypeId", value.clone())?;
        let pattern = get_string_key("pattern", value.clone())?;

        Ok(ClassifyingRule {
            id,
            name,
            transaction_type_id,
            pattern,
        })
    }
}

fn classifying_rule_list_to_attributes(
    list: ClassifyingRuleList,
) -> HashMap<String, AttributeValue> {
    let rules: Vec<AttributeValue> = list
        .into_iter()
        .map(|rule| AttributeValue::M(rule.into()))
        .collect();

    let mut map = HashMap::new();
    map.insert("name".to_string(), AttributeValue::S("RULES".to_string()));
    map.insert("rules".to_string(), AttributeValue::L(rules));
    map
}

fn attributes_to_classifying_rule_list(
    values: HashMap<String, AttributeValue>,
) -> Result<ClassifyingRuleList, FinError> {
    let rule_list = get_list_key("rules", values)?;
    let mut rules: Vec<ClassifyingRule> = Vec::new();
    for rule in rule_list.into_iter() {
        let rule = rule.as_m();
        if rule.is_err() {
            continue;
        }

        let rule = rule.expect("should handle err case").clone();
        let rule: Result<ClassifyingRule, _> = rule.try_into();
        if rule.is_err() {
            continue;
        }
        let rule = rule.expect("should handle err case");
        rules.push(rule);
    }
    Ok(rules)
}

impl FinDynamoDb {
    async fn save_classifying_rules(&self, rules: ClassifyingRuleList) -> Result<(), FinError> {
        let put_builder = self.client.put_item();
        put_builder
            .table_name(&self.classifying_rules_tablename)
            .set_item(Some(classifying_rule_list_to_attributes(rules)))
            .send()
            .await?;
        Ok(())
    }
}

#[async_trait]
impl ClassifyingRuleRepository for FinDynamoDb {
    async fn get_all(
        &self,
        _pagination: Option<PaginationDetails>,
    ) -> Result<ClassifyingRuleList, FinError> {
        let builder = self.client.get_item();
        let result = builder
            .table_name(&self.classifying_rules_tablename.to_string())
            .key("name", AttributeValue::S("RULES".to_string()))
            .send()
            .await?;

        let attributes = result.item();
        if let None = attributes {
            return Ok(vec![]);
        }
        let attributes = attributes.expect("Should handle none case");
        Ok(attributes_to_classifying_rule_list(attributes.clone())?)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError> {
        let rules = ClassifyingRuleRepository::get_all(self, None).await?;
        Ok(rules.into_iter().find(|rule| rule.id == id))
    }

    async fn create(
        &self,
        classifying_rule: ClassifyingRuleCreationArgs,
    ) -> Result<ClassifyingRule, FinError> {
        let mut current = ClassifyingRuleRepository::get_all(self, None).await?;
        let rule = classifying_rule.to_rule(Uuid::new_v4().to_string());
        current.insert(0, rule.clone());
        self.save_classifying_rules(current).await?;
        Ok(rule)
    }

    async fn update(
        &self,
        classifying_rule: ClassifyingRuleUpdateArgs,
    ) -> Result<ClassifyingRule, FinError> {
        let mut rules = ClassifyingRuleRepository::get_all(self, None).await?;
        let mut index: Option<usize> = None;
        for (i, rule) in rules.iter().enumerate() {
            if rule.id == classifying_rule.id {
                index = Some(i);
                break;
            }
        }
        let index = index.ok_or(FinError::NotFound(format!(
            "Rule with id {}",
            classifying_rule.id
        )))?;
        let existing_rule = rules.get(index).expect("Should handle check above").clone();
        let new_rule = classifying_rule.merge_existing(existing_rule);
        _ = std::mem::replace(&mut rules[index], new_rule.clone());
        self.save_classifying_rules(rules).await?;
        Ok(new_rule)
    }

    async fn delete(&self, id: &str) -> Result<ClassifyingRule, FinError> {
        let rules = ClassifyingRuleRepository::get_all(self, None).await?;
        let deleted_rule = rules
            .iter()
            .filter(|rule| rule.id == id)
            .next()
            .ok_or(FinError::NotFound(format!("Rule with id {}", id)))?
            .clone();
        let new_rules = rules.into_iter().filter(|rule| rule.id != id).collect();
        self.save_classifying_rules(new_rules).await?;
        Ok(deleted_rule)
    }

    async fn reorder(
        &self,
        id: &str,
        after: Option<&str>,
    ) -> Result<ClassifyingRuleList, FinError> {
        let mut rules = ClassifyingRuleRepository::get_all(self, None).await?;

        // If after is none, move it to the top
        if let None = after {
            let mut index: Option<usize> = None;
            for (i, rule) in rules.iter().enumerate() {
                if rule.id == id {
                    index = Some(i);
                    break;
                }
            }
            let index = index.ok_or(FinError::NotFound(format!("Rule with id {}", id)))?;
            let rule = rules.remove(index);
            rules.insert(0, rule);
            self.save_classifying_rules(rules).await?;
            return Ok(ClassifyingRuleRepository::get_all(self, None).await?);
        }

        // Otherwise, move it after the specified "after rule"
        let after = after.expect("Should handle none case");
        if after == id {
            return Ok(rules);
        }
        let mut index: Option<usize> = None;
        let mut after_index: Option<usize> = None;
        for (i, rule) in rules.iter().enumerate() {
            if rule.id == id {
                index = Some(i);
            }
            if rule.id == after {
                after_index = Some(i);
            }
        }
        let index = index.ok_or(FinError::NotFound(format!("Rule with id {}", id)))?;
        let after_index =
            after_index.ok_or(FinError::NotFound(format!("Rule with id {}", after)))?;

        if after_index == index {
            return Ok(rules);
        }

        let rule = rules.remove(index);
        if after > id {
            rules.insert(after_index, rule);
        } else {
            rules.insert(after_index + 1, rule);
        }
        self.save_classifying_rules(rules).await?;
        Ok(ClassifyingRuleRepository::get_all(self, None).await?)
    }
}

#[async_trait]
impl UnprocessedTransactionRepository for FinDynamoDb {
    async fn get_all(
        &self,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError> {
        let builder = self.client.scan();
        let result: Result<Vec<HashMap<String, AttributeValue>>, _> = builder
            .table_name(&self.unprocessed_transactions_tablename.to_string())
            .into_paginator()
            .items()
            .send()
            .collect()
            .await;

        let result = result?;
        let mut transactions: Vec<UnprocessedTransaction> = Vec::new();
        for item in result {
            if let Ok(transaction) = item.try_into() {
                transactions.push(transaction);
            }
        }
        transactions.sort();
        transactions.reverse();
        Ok(transactions)
    }

    async fn create(
        &self,
        transaction: UnprocessedTransaction,
    ) -> Result<UnprocessedTransaction, FinError> {
        let builder = self.client.put_item();
        let result = builder
            .table_name(self.unprocessed_transactions_tablename.to_string())
            .set_item(Some(transaction.clone().into()))
            .condition_expression("attribute_not_exists(id)")
            .send()
            .await;

        if let Err(SdkError::ServiceError(put_item_err)) = result {
            let err = put_item_err.err();
            if let PutItemError::ConditionalCheckFailedException(_) = err {
                return Err(FinError::AlreadyExists(format!(
                    "UnproccessedTransaction with id {}",
                    transaction.id
                )));
            }
        }
        Ok(transaction)
    }
    async fn delete(&self, id: &str) -> Result<UnprocessedTransaction, FinError> {
        let builder = self.client.delete_item();
        let result = builder
            .table_name(self.unprocessed_transactions_tablename.to_string())
            .key("id", AttributeValue::S(id.to_string()))
            .return_values(ReturnValue::AllOld)
            .send()
            .await?;

        let item = result.attributes.ok_or(FinError::NotFound(format!(
            "UnproccessedTransaction with id {}",
            id
        )))?;

        Ok(item.try_into()?)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<UnprocessedTransaction>, FinError> {
        let builder = self.client.get_item();
        let result = builder
            .table_name(self.unprocessed_transactions_tablename.to_string())
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await?;

        let item = result.item;
        if let Some(item) = item {
            Ok(Some(item.try_into()?))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl TransactionRepository for FinDynamoDb {
    async fn get_by_id(&self, id: &str) -> Result<Option<Transaction>, FinError> {
        let builder = self.client.get_item();
        let result = builder
            .table_name(self.transactions_tablename.to_string())
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await?;
        let attributes = result.item;
        if let Some(attributes) = attributes {
            Ok(Some(attributes.try_into()?))
        } else {
            Ok(None)
        }
    }
    async fn get_by_month(
        &self,
        month: i64,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<Transaction>, FinError> {
        let query_builder = self.client.query();
        let result: Result<Vec<HashMap<String,AttributeValue>>, _> = 
            query_builder.table_name(self.transactions_tablename.to_string())
            .key_condition_expression("month = :month")
            .index_name("month-date-index")
            .expression_attribute_values("month", AttributeValue::N(month.to_string()))
            .into_paginator()
            .items()
            .send()
            .collect()
            .await;

        let result = result?;
        let mut transactions: Vec<Transaction> = Vec::new();
        for item in result {
            if let Ok(transaction) = item.try_into() {
                transactions.push(transaction);
            }
        }
        transactions.sort();
        transactions.reverse();
        Ok(transactions)
    }
    async fn create(&self, transaction: Transaction) -> Result<Transaction, FinError> {
        let builder = self.client.put_item();
        let result = builder
            .table_name(self.transactions_tablename.to_string())
            .set_item(Some(transaction.clone().into()))
            .condition_expression("attribute_not_exists(id)")
            .send()
            .await;

        if let Err(SdkError::ServiceError(put_item_err)) = result {
            let err = put_item_err.err();
            if let PutItemError::ConditionalCheckFailedException(_) = err {
                return Err(FinError::AlreadyExists(format!(
                    "Transaction with id {}",
                    transaction.id
                )));
            }
        }
        Ok(transaction)
    }
}
