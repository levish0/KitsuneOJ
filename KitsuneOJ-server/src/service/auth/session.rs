use crate::config::db_config::DbConfig;
use crate::dto::auth::internal::session::Session;
use crate::errors::errors::Errors;
use redis::aio::ConnectionManager as RedisClient;
use redis::AsyncCommands;
use serde_json;

pub struct SessionService;

impl SessionService {
    pub async fn create_session(
        redis: &RedisClient,
        user_id: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<Session, Errors> {
        let config = DbConfig::get();
        let session = Session::new(user_id, config.auth_session_expire_time)
            .with_client_info(user_agent, ip_address);

        let session_data = serde_json::to_string(&session).map_err(|e| {
            Errors::SysInternalError(format!("Session serialization failed: {}", e))
        })?;

        let mut conn = redis.clone();
        let ttl_seconds = (config.auth_session_expire_time * 60) as u64;

        conn.set_ex::<_, _, ()>(&session.redis_key(), session_data, ttl_seconds)
            .await
            .map_err(|e| {
                Errors::SysInternalError(format!("Redis session creation failed: {}", e))
            })?;

        Ok(session)
    }

    pub async fn get_session(
        redis: &RedisClient,
        session_id: &str,
    ) -> Result<Option<Session>, Errors> {
        let mut conn = redis.clone();
        let key = format!("session:{}", session_id);

        let session_data: Option<String> = conn.get(&key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis session retrieval failed: {}", e))
        })?;

        match session_data {
            Some(data) => {
                let session: Session = serde_json::from_str(&data).map_err(|e| {
                    Errors::SysInternalError(format!("Session deserialization failed: {}", e))
                })?;

                if session.is_expired() {
                    Self::delete_session(redis, session_id).await?;
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            }
            None => Ok(None),
        }
    }

    pub async fn delete_session(redis: &RedisClient, session_id: &str) -> Result<(), Errors> {
        let mut conn = redis.clone();
        let key = format!("session:{}", session_id);

        conn.del::<_, ()>(&key).await.map_err(|e| {
            Errors::SysInternalError(format!("Redis session deletion failed: {}", e))
        })?;

        Ok(())
    }

    pub async fn refresh_session(
        redis: &RedisClient,
        session_id: &str,
    ) -> Result<Option<Session>, Errors> {
        if let Some(mut session) = Self::get_session(redis, session_id).await? {
            let config = DbConfig::get();
            let now = chrono::Utc::now();
            session.expires_at = now + chrono::Duration::minutes(config.auth_session_expire_time);

            let session_data = serde_json::to_string(&session).map_err(|e| {
                Errors::SysInternalError(format!("Session serialization failed: {}", e))
            })?;

            let mut conn = redis.clone();
            let ttl_seconds = (config.auth_session_expire_time * 60) as u64;

            conn.set_ex::<_, _, ()>(&session.redis_key(), session_data, ttl_seconds)
                .await
                .map_err(|e| {
                    Errors::SysInternalError(format!("Redis session refresh failed: {}", e))
                })?;

            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub async fn cleanup_expired_sessions(
        redis: &RedisClient,
        user_id: &str,
    ) -> Result<(), Errors> {
        let mut conn = redis.clone();
        let pattern = "session:*";

        let keys: Vec<String> = conn
            .keys(pattern)
            .await
            .map_err(|e| Errors::SysInternalError(format!("Redis key scan failed: {}", e)))?;

        for key in keys {
            if let Ok(Some(session_data)) = conn.get::<_, Option<String>>(&key).await {
                if let Ok(session) = serde_json::from_str::<Session>(&session_data) {
                    if session.user_id == user_id && session.is_expired() {
                        let _ = conn.del::<_, ()>(&key).await;
                    }
                }
            }
        }

        Ok(())
    }
}