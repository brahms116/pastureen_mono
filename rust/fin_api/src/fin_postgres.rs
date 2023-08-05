use super::*;
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub struct FinPostgres {
    pool: PgPool,
}

impl FinPostgres {
    pub async fn new() -> Result<Self, FinError> {
        let uri =
            std::env::var("FIN_POSTGRES_URI").map_err(|e| FinError::DbError(format!("{e:?}")))?;
        let pool = PgPoolOptions::new()
            .connect(&uri)
            .await
            .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl TransactionRepository for FinPostgres {
    async fn get_by_id(&self, id: &str) -> Result<Option<Transaction>, FinError> {
        let res = sqlx::query_as!(
            Transaction,
            "SELECT 
                id,
                amount_cents,
                description,
                date,
                transaction_type_id::text as \"transaction_type_id!\"
            FROM transaction WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;

        Ok(res)
    }

    async fn get_by_month(
        &self,
        _month: i64,
        _pagintaion: Option<PaginationDetails>,
    ) -> Result<Vec<Transaction>, FinError> {
        todo!()
    }
    async fn create(&self, transaction: Transaction) -> Result<Transaction, FinError> {
        let res = sqlx::query_as!(
            Transaction,
            "INSERT INTO transaction (
                id, amount_cents, description, date, transaction_type_id
            ) 
            VALUES ($1, $2, $3, $4, $5::text::uuid)
            RETURNING 
                id,
                amount_cents,
                description,
                date,
                transaction_type_id::text as \"transaction_type_id!\"
            ",
            transaction.id,
            i32::try_from(transaction.amount_cents).unwrap(),
            transaction.description,
            i32::try_from(transaction.date).unwrap(),
            transaction.transaction_type_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }
}

#[async_trait]
impl UnprocessedTransactionRepository for FinPostgres {
    async fn get_all(
        &self,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<UnprocessedTransaction>, FinError> {
        let res = sqlx::query_as!(
            UnprocessedTransaction,
            "SELECT 
                id,
                amount_cents,
                description,
                date
            FROM unprocessed_transaction ORDER BY date DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;

        Ok(res)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<UnprocessedTransaction>, FinError> {
        let res = sqlx::query_as!(
            UnprocessedTransaction,
            "SELECT 
                id,
                amount_cents,
                description,
                date
            FROM unprocessed_transaction WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;

        Ok(res)
    }

    async fn create(
        &self,
        args: UnprocessedTransaction,
    ) -> Result<UnprocessedTransaction, FinError> {
        let res = sqlx::query_as!(
            UnprocessedTransaction,
            "INSERT INTO unprocessed_transaction (
                id, amount_cents, description, date
            ) VALUES ($1, $2, $3, $4) RETURNING id, amount_cents, description, date",
            args.id,
            i32::try_from(args.amount_cents).unwrap(),
            args.description,
            i32::try_from(args.date).unwrap()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }

    async fn delete(&self, id: &str) -> Result<UnprocessedTransaction, FinError> {
        let res = sqlx::query_as!(
            UnprocessedTransaction,
            "DELETE FROM 
                unprocessed_transaction 
            WHERE id = $1 
            RETURNING id, amount_cents, description, date
            ",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }
}

#[async_trait]
impl ClassifyingRuleRepository for FinPostgres {
    async fn get_all(
        &self,
        _pagination: Option<PaginationDetails>,
    ) -> Result<ClassifyingRuleList, FinError> {
        let res = sqlx::query_as!(
            ClassifyingRule,
            "SELECT 
                id::text as \"id!\",
                name,
                match_string as \"pattern\",
                transaction_type_id::text as \"transaction_type_id!\"
            FROM match_rule ORDER BY order_index ASC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;

        Ok(res)
    }
    async fn get_by_id(&self, id: &str) -> Result<Option<ClassifyingRule>, FinError> {
        let uuid = Uuid::parse_str(id);
        if let Err(_) = uuid {
            return Ok(None);
        }
        let uuid = uuid.expect("Should handle err case");
        let res = sqlx::query_as!(
            ClassifyingRule,
            "SELECT 
                id::text as \"id!\",
                name,
                match_string as \"pattern\",
                transaction_type_id::text as \"transaction_type_id!\"
            FROM match_rule WHERE id = $1",
            uuid
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }

    async fn create(&self, args: ClassifyingRuleCreationArgs) -> Result<ClassifyingRule, FinError> {
        let res = sqlx::query_as!(
            ClassifyingRule,
            "INSERT INTO match_rule (
                name, match_string, order_index ,transaction_type_id
            ) VALUES ($1, $2, 
                (SELECT COUNT(*)+1 FROM match_rule),
                $3::text::uuid) 
            RETURNING 
                id::text as \"id!\",
                name,
                match_string as \"pattern\",
                transaction_type_id::text as \"transaction_type_id!\"
            ",
            args.name,
            args.pattern,
            args.transaction_type_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }

    async fn update(&self, rule: ClassifyingRuleUpdateArgs) -> Result<ClassifyingRule, FinError> {
        let existing = ClassifyingRuleRepository::get_by_id(self, &rule.id).await?;

        if existing.is_none() {
            return Err(FinError::NotFound(format!(
                "Classifying rule with id {}",
                rule.id
            )));
        }

        let existing = existing.unwrap();
        let new_rule = ClassifyingRule {
            id: rule.id,
            name: rule.name.unwrap_or(existing.name),
            pattern: rule.pattern.unwrap_or(existing.pattern),
            transaction_type_id: rule
                .transaction_type_id
                .unwrap_or(existing.transaction_type_id),
        };

        let new_uuid =
            Uuid::parse_str(&new_rule.id).expect("Should never happen as this is pulled from db");

        let res = sqlx::query_as!(
            ClassifyingRule,
            "UPDATE 
                match_rule SET name = $1,
                match_string = $2,
                transaction_type_id = $3::text::uuid 
            WHERE 
                id = $4 
            RETURNING 
                id::text as \"id!\",
                name,
                match_string as \"pattern\",
                transaction_type_id::text as \"transaction_type_id!\"
            ",
            new_rule.name,
            new_rule.pattern,
            new_rule.transaction_type_id,
            new_uuid
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }

    async fn delete(&self, id: &str) -> Result<ClassifyingRule, FinError> {
        let uuid = Uuid::parse_str(id);
        if let Err(_) = uuid {
            return Err(FinError::NotFound(format!(
                "Classifying rule with id {}",
                id
            )));
        }
        let res = sqlx::query_as!(
            ClassifyingRule,
            "DELETE FROM match_rule WHERE id = $1 RETURNING 
                id::text as \"id!\",
                name,
                match_string as \"pattern\",
                transaction_type_id::text as \"transaction_type_id!\"
            ",
            uuid.expect("Should handle err case")
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;

        if res.is_none() {
            return Err(FinError::NotFound(format!(
                "Classifying rule with id {}",
                id
            )));
        }
        Ok(res.expect("Should handle none case"))
    }

    async fn reorder(
        &self,
        id: &str,
        after: Option<&str>,
    ) -> Result<ClassifyingRuleList, FinError> {
        ClassifyingRuleRepository::get_all(self, None).await
    }
}

#[async_trait]
impl TransactionTypeRepository for FinPostgres {
    async fn get_all(
        &self,
        _pagination: Option<PaginationDetails>,
    ) -> Result<Vec<TransactionType>, FinError> {
        let res = sqlx::query_as!(
            TransactionType,
            "SELECT 
                id::text as \"id!\",
                name FROM transaction_type"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<TransactionType>, FinError> {
        let uuid = Uuid::parse_str(id).ok();

        if let Some(uuid) = uuid {
            let res = sqlx::query_as!(
                TransactionType,
                "SELECT 
                id::text as \"id!\",
                name FROM transaction_type WHERE id = $1",
                uuid
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| FinError::DbError(format!("{e:?}")))?;
            return Ok(res);
        }
        Ok(None)
    }

    async fn create(&self, name: &str) -> Result<TransactionType, FinError> {
        let res = sqlx::query_as!(
            TransactionType,
            "INSERT INTO transaction_type (name) VALUES ($1) RETURNING id::text as \"id!\", name",
            name
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }

    async fn update(&self, transaction_type: TransactionType) -> Result<TransactionType, FinError> {
        let uuid = Uuid::parse_str(&transaction_type.id).map_err(|_| {
            FinError::NotFound(format!("Transaction type with id {}", transaction_type.id))
        })?;
        let res = sqlx::query_as!(
            TransactionType,
            "UPDATE transaction_type SET name = $1 WHERE id = $2 RETURNING id::text as \"id!\", name",
            transaction_type.name,
            uuid
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| FinError::DbError(format!("{e:?}")))?;
        Ok(res)
    }
}
