use crate::domain::clipboard_item::{ClipboardContentType, ClipboardItem};
use crate::domain::clipboard_repository::{ClipboardRepository, RepositoryError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};
use std::result::Result;

#[derive(Debug)]
pub struct SqliteClipboardRepository {
    pool: SqlitePool,
}

impl SqliteClipboardRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl ClipboardRepository for SqliteClipboardRepository {
    async fn save_item(&self, item: ClipboardItem) -> Result<(), RepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(RepositoryError::Connection)?;
        let content_type = item.content_type as i32; // Assuming you have a conversion here
        sqlx::query!(
            r#"
            INSERT INTO clipboard_items (id, content_type, content)
            VALUES ($1, $2, $3)
            "#,
            item.id,
            content_type,
            item.content
        )
        .execute(&mut conn)
        .await
        .map_err(RepositoryError::Query)?;
        Ok(())
    }

    async fn get_item(&self, id: u32) -> Result<Option<ClipboardItem>, RepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(RepositoryError::Connection)?;
        let res = sqlx::query!(
            r#"
            SELECT id, content_type, content
            FROM clipboard_items
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&mut conn)
        .await
        .map_err(RepositoryError::Query)?;

        if let Some(row) = res {
            let content_type = ClipboardContentType::from_i32(row.content_type) // Assuming you have a conversion here
                .ok_or(RepositoryError::Conversion(
                    "Invalid content type".to_string(),
                ))?;
            Ok(Some(ClipboardItem {
                id: row.id,
                content_type,
                content: row.content,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_all_items(&self) -> Result<Vec<ClipboardItem>, RepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(RepositoryError::Connection)?;
        let rows = sqlx::query!(
            r#"
            SELECT id, content_type, content
            FROM clipboard_items
            "#
        )
        .fetch_all(&mut conn)
        .await
        .map_err(RepositoryError::Query)?;

        let mut items = Vec::new();
        for row in rows {
            let content_type = ClipboardContentType::from_i32(row.content_type) // Assuming you have a conversion here
                .ok_or(RepositoryError::Conversion(
                    "Invalid content type".to_string(),
                ))?;
            items.push(ClipboardItem {
                id: row.id,
                content_type,
                content: row.content,
            });
        }
        Ok(items)
    }
}
