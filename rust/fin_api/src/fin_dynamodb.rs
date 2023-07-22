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

impl<E: Error> From<SdkError<E>> for FinError {
    fn from(err: SdkError<E>) -> Self {
        Self::DbError(format!("{:?}", err))
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
}

#[async_trait]
impl TransactionTypeRepository for FinDynamoDb {
    async fn get_all(&self) -> Result<Vec<TransactionType>, FinError> {
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
    async fn get_all(&self) -> Result<ClassifyingRuleList, FinError> {
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
        let rules = ClassifyingRuleRepository::get_all(self).await?;
        Ok(rules.into_iter().find(|rule| rule.id == id))
    }

    async fn create(
        &self,
        classifying_rule: ClassifyingRuleCreationArgs,
    ) -> Result<ClassifyingRule, FinError> {
        let mut current = ClassifyingRuleRepository::get_all(self).await?;
        let rule = classifying_rule.to_rule(Uuid::new_v4().to_string());
        current.insert(0, rule.clone());
        self.save_classifying_rules(current).await?;
        Ok(rule)
    }

    async fn update(
        &self,
        classifying_rule: ClassifyingRuleUpdateArgs,
    ) -> Result<ClassifyingRule, FinError> {
        let mut rules = ClassifyingRuleRepository::get_all(self).await?;
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
        let rules = ClassifyingRuleRepository::get_all(self).await?;
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

    async fn reorder(&self, id: &str, after: &str) -> Result<ClassifyingRuleList, FinError> {
        let mut rules = ClassifyingRuleRepository::get_all(self).await?;
        if id == after {
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
        Ok(ClassifyingRuleRepository::get_all(self).await?)
    }
}
