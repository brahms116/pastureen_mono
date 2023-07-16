use super::*;
use async_trait::async_trait;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::error::Error;
use tokio_stream::StreamExt;
use uuid::Uuid;

impl<E: Error> From<SdkError<E>> for FinError {
    fn from(err: SdkError<E>) -> Self {
        Self::DbError(err.to_string())
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

        let update_builder = self.client.update_item();
        let result = update_builder
            .table_name(&self.transaction_type_tablename)
            .set_expression_attribute_values(Some(transaction_type.into()))
            .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
            .send()
            .await?;

        let item = result.attributes.ok_or(FinError::DbError(
            "Could not find attributes in result".to_string(),
        ))?;

        Ok(TransactionType::try_from(item)?)
    }

    async fn update(&self, transaction_type: TransactionType) -> Result<TransactionType, FinError> {
        let update_builder = self.client.update_item();
        let result = update_builder
            .table_name(&self.transaction_type_tablename)
            .set_expression_attribute_values(Some(transaction_type.into()))
            .return_values(aws_sdk_dynamodb::types::ReturnValue::AllNew)
            .send()
            .await?;

        let item = result.attributes.ok_or(FinError::DbError(
            "Could not find attributes in result".to_string(),
        ))?;

        Ok(TransactionType::try_from(item)?)
    }
}
