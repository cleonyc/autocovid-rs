//     Copyright (C) 2022  cleonyc
// Licensed under GNU Affero General Public License. https://www.gnu.org/licenses/agpl-3.0.en.html

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
#[derive(Clone, Debug)]
pub struct Database {
    pool: Pool<Postgres>,
}
#[derive(sqlx::Type, Clone)]
#[sqlx(type_name = "command_type", rename_all = "snake_case")]
pub enum CommandType {
    SLASH,
    TEXT,
    BOTH,
}
#[derive(sqlx::FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub discord_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub school_code: String,
}
//TODO: make settings work
#[derive(sqlx::FromRow, Clone)]
pub struct Setting {
    server_id: i64,
    command_type: CommandType,
    active_channels: Option<Vec<i64>>,
}
impl Database {
    pub async fn init(url: &str, first_start: bool) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
        if first_start {
            sqlx::query!("CREATE TYPE command_type AS ENUM ('slash', 'text', 'both')")
                .execute(&pool)
                .await?;
            sqlx::query!(
                "CREATE TABLE IF NOT EXISTS users (\
                id SERIAL PRIMARY KEY, \
                discord_id BIGINT NOT NULL, \
                first_name TEXT NOT NULL, \
                last_name TEXT NOT NULL, \
                email TEXT NOT NULL, \
                school_code TEXT NOT NULL);"
            )
            .execute(&pool)
            .await?;
            sqlx::query!(
                "CREATE TABLE IF NOT EXISTS settings (\
                id SERIAL PRIMARY KEY, \
                command_type command_type NOT NULL, \
                server_id BIGINT NOT NULL, \
                active_channels BIGINT[]\
                );"
            )
            .execute(&pool)
            .await?;
        }
        Ok(Self { pool })
    }
    pub async fn new_user(
        &self,
        discord_id: u64,
        first_name: String,
        last_name: String,
        email: String,
        school_code: String,
    ) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO users (discord_id, first_name, last_name, email, school_code) VALUES ($1, $2, $3, $4, $5)", discord_id as i64, first_name, last_name, email, school_code)
            .execute(&self.pool).await?;
        Ok(())
    }
    pub async fn new_server(
        &self,
        server_id: u64,
        command_type: CommandType,
        active_channels: Vec<u64>,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO settings (server_id, command_type, active_channels) VALUES ($1, $2, $3)",
            server_id as i64,
            command_type as CommandType,
            &active_channels
                .iter()
                .map(|x| *x as i64)
                .collect::<Vec<i64>>()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn get_user(&self, discord_id: u64) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE discord_id = $1",
            discord_id as i64
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
    pub async fn get_all_users(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }
    pub async fn get_settings(&self, server_id: u64) -> anyhow::Result<Setting> {
        let settings = sqlx::query_as!(
            Setting,
            "SELECT server_id, command_type as \"command_type: CommandType\", active_channels FROM settings WHERE server_id = $1",
            server_id as i64
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(settings.unwrap())
    }
    // pub async fn get_prefix(&self, server_id:  u64) -> anyhow::Result<String> {
    //     let rec = sqlx::query!(
    //         "SELECT prefix FROM settings WHERE server_id = $1",
    //         server_id as i64
    //     )
    //         .fetch_one(&self.pool)
    //         .await?;
    //     Ok(rec.prefix.unwrap())
    // }
    pub async fn modify_user(
        &self,
        discord_id: u64,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        school_code: Option<String>,
    ) -> anyhow::Result<()> {
        if first_name.is_some() && last_name.is_some() {
            sqlx::query!(
                "UPDATE users SET first_name = $1, last_name = $2 WHERE discord_id = $3",
                first_name,
                last_name,
                discord_id as i64
            )
            .execute(&self.pool)
            .await?;
        }
        if email.is_some() {
            sqlx::query!(
                "UPDATE users SET email = $1 WHERE discord_id = $2",
                email,
                discord_id as i64
            )
            .execute(&self.pool)
            .await?;
        }
        if school_code.is_some() {
            sqlx::query!(
                "UPDATE users SET school_code = $1 WHERE discord_id = $2",
                school_code,
                discord_id as i64
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
    pub async fn modify_settings(
        &self,
        server_id: u64,
        command_type: Option<CommandType>,
        active_channels: Option<Vec<u64>>,
    ) -> anyhow::Result<()> {
        let mut ref_vec: Vec<i64> = vec![];
        let act_channels = match active_channels {
            None => None,
            Some(i) => {
                ref_vec = i.iter().map(|x| *x as i64).collect::<Vec<i64>>();
                Some(ref_vec.as_slice())
            }
        };
        sqlx::query!(
            "UPDATE settings SET command_type = $1, active_channels = $2 WHERE server_id = $3",
            command_type as Option<CommandType>,
            act_channels,
            server_id as i64
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
