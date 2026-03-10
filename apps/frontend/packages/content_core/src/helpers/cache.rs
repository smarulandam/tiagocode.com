use redis::{AsyncTypedCommands, Client};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::future::Future;
use std::time::Duration;

use crate::application::domain::core::{AppError, Result};

#[derive(Clone, Debug)]
pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn remember<T, F, Fut>(&self, key: &str, ttl: Duration, producer: F) -> Result<T>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let key = String::from(key);

        let mut connection = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::external("redis", e))?;

        let cached: Option<String> = connection
            .get(key.as_str())
            .await
            .map_err(|e| AppError::external("redis", e))?;

        if let Some(raw) = cached {
            let value =
                serde_json::from_str::<T>(&raw).map_err(|e| AppError::decode("redis", e))?;

            return Ok(value);
        }

        let value = producer().await?;

        let raw = serde_json::to_string(&value).map_err(|e| AppError::encode("redis", e))?;

        if ttl.is_zero() {
            let _: () = connection
                .set(&key, raw)
                .await
                .map_err(|e| AppError::external("redis", e))?;
        } else {
            let _: () = connection
                .set_ex(&key, raw, ttl.as_secs())
                .await
                .map_err(|e| AppError::external("redis", e))?;
        }

        Ok(value)
    }

    pub async fn clear_all(&self) -> Result<()> {
        let mut connection = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::external("redis", e))?;

        let _: () = redis::cmd("FLUSHDB")
            .query_async(&mut connection)
            .await
            .map_err(|e| AppError::external("redis", e))?;

        Ok(())
    }
}
