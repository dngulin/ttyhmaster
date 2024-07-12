#[cfg(feature = "sqlite")]
use sqlx::Sqlite as Database;

#[cfg(feature = "mysql")]
use sqlx::MySql as Database;

pub type DbPool = sqlx::Pool<Database>;
pub type DbTransaction<'a> = sqlx::Transaction<'a, Database>;

pub struct DbQueryError;

pub enum DbLookupError {
    NotFound,
    QueryError,
}

pub struct NewPlayerInfo<'a> {
    pub player_name: &'a str,
    pub player_id: &'a str,
    pub password_hash: &'a str,
    pub salt: &'a str,
    pub access_token: &'a str,
}

pub async fn create_player<'a>(
    db_pool: &DbPool,
    info: NewPlayerInfo<'a>,
) -> Result<(), DbQueryError> {
    sqlx::query!(
        r#"
            INSERT INTO players (
                player_name, player_id,
                password_hash, salt,
                access_token
            )
            VALUES( ?, ?, ?, ?, ? )
        "#,
        info.player_name,
        info.player_id,
        info.password_hash,
        info.salt,
        info.access_token
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub(crate) async fn set_password<'a>(
    transaction: &mut DbTransaction<'a>,
    player_name: &str,
    password_hash: &str,
    password_salt: &str,
) -> Result<(), DbLookupError> {
    sqlx::query!(
        "UPDATE players SET password_hash = ?, salt = ? WHERE player_name = ?",
        password_hash,
        password_salt,
        player_name
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

pub(crate) async fn set_is_mojang<'a>(
    transaction: &mut DbTransaction<'a>,
    player_name: &str,
    is_mojang: bool,
) -> Result<(), DbLookupError> {
    sqlx::query!(
        "UPDATE players SET is_mojang = ? WHERE player_name = ?",
        is_mojang,
        player_name
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

pub(crate) async fn set_player_name<'a>(
    transaction: &mut DbTransaction<'a>,
    player_name: &str,
    new_player_name: &str,
) -> Result<(), DbLookupError> {
    sqlx::query!(
        "UPDATE players SET player_name = ? WHERE player_name = ?",
        new_player_name,
        player_name
    )
    .execute(&mut **transaction)
    .await?;

    Ok(())
}

pub struct PlayerAuthRecord {
    pub player_id: String,
    pub is_mojang: bool,
    pub password_hash: String,
    pub salt: String,
}

pub async fn get_player_auth_record(
    db_pool: &DbPool,
    name: &str,
) -> Result<PlayerAuthRecord, DbLookupError> {
    let record = sqlx::query_as!(
        PlayerAuthRecord,
        r#"
        SELECT
            player_id,
            is_mojang as `is_mojang:_`,
            password_hash,
            salt
        FROM players
        WHERE player_name = ?"#,
        name
    )
    .fetch_one(db_pool)
    .await?;

    Ok(record)
}

pub async fn set_access_token(
    db_pool: &DbPool,
    player_name: &str,
    access_token: &str,
) -> Result<(), DbQueryError> {
    sqlx::query!(
        "UPDATE players SET access_token = ? WHERE player_name = ?",
        access_token,
        player_name
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub struct Stats<'a> {
    pub player_name: &'a str,
    pub launcher_version: &'a str,
    pub os: &'a str,
    pub os_version: &'a str,
    pub os_word_size: &'a str,
    pub install_uuid: &'a str,
    pub machine_uuid: &'a str,
}

pub async fn collect_stats<'a>(db_pool: &DbPool, stats: Stats<'a>) -> Result<(), DbQueryError> {
    sqlx::query!(
        r#"
            INSERT INTO stats (
                player_name, launcher_ver,
                os, os_version, os_word_size,
                install_uuid, machine_uuid
            )
            VALUES( ?, ?, ?, ?, ?, ?, ? )
        "#,
        stats.player_name,
        stats.launcher_version,
        stats.os,
        stats.os_version,
        stats.os_word_size,
        stats.install_uuid,
        stats.machine_uuid
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub async fn set_skin_model(
    db_pool: &DbPool,
    player_name: &str,
    is_slim: bool,
) -> Result<(), DbQueryError> {
    sqlx::query!(
        "UPDATE players SET is_slim_model = ? WHERE player_name = ?",
        is_slim,
        player_name
    )
    .execute(db_pool)
    .await?;

    Ok(())
}

pub struct PlayerJoinRecord {
    pub access_token: String,
    pub is_mojang: bool,
}

pub async fn get_player_join_record(
    db_pool: &DbPool,
    player_id: &str,
) -> Result<PlayerJoinRecord, DbLookupError> {
    let record = sqlx::query_as!(
        PlayerJoinRecord,
        "SELECT access_token, is_mojang as `is_mojang:_` FROM players WHERE player_id = ?",
        player_id
    )
    .fetch_one(db_pool)
    .await?;

    Ok(record)
}

pub struct PlayerRecord {
    pub player_name: String,
    pub player_id: String,
    pub is_mojang: bool,
    pub is_slim_model: bool,
}

pub async fn get_player_record_by_name(
    db_pool: &DbPool,
    player_name: &str,
) -> Result<PlayerRecord, DbLookupError> {
    let record = sqlx::query_as!(
        PlayerRecord,
        r#"
        SELECT
            player_name, player_id,
            is_mojang as `is_mojang:_`,
            is_slim_model as `is_slim_model:_` 
        FROM players
        WHERE player_name = ?"#,
        player_name
    )
    .fetch_one(db_pool)
    .await?;

    Ok(record)
}

pub async fn get_player_record_by_id(
    db_pool: &DbPool,
    player_id: &str,
) -> Result<PlayerRecord, DbLookupError> {
    let record = sqlx::query_as!(
        PlayerRecord,
        r#"
        SELECT
            player_name, player_id,
            is_mojang as `is_mojang:_`,
            is_slim_model as `is_slim_model:_`
        FROM players
        WHERE player_id = ?"#,
        player_id
    )
    .fetch_one(db_pool)
    .await?;

    Ok(record)
}

pub async fn get_player_id_by_access_token(
    db_pool: &DbPool,
    access_token: &str,
) -> Result<String, DbLookupError> {
    let record = sqlx::query!(
        "SELECT player_id FROM players WHERE access_token = ?",
        access_token
    )
    .fetch_one(db_pool)
    .await?;

    Ok(record.player_id)
}

mod map_error {
    use super::{DbLookupError, DbQueryError};
    use sqlx::Error;

    impl From<Error> for DbQueryError {
        fn from(error: Error) -> Self {
            tracing::error!("DbQueryError: {error}");
            Self
        }
    }

    impl From<Error> for DbLookupError {
        fn from(error: Error) -> Self {
            match error {
                Error::RowNotFound => DbLookupError::NotFound,
                _ => {
                    tracing::error!("DbLookupError: {error}");
                    DbLookupError::QueryError
                }
            }
        }
    }
}
