use std::path::PathBuf;
use poise::serenity_prelude::{Channel, ChannelId, Guild, GuildId};
use sqlx::{Connection, SqliteConnection, SqlitePool};

use crate::markov::scanner::{BungusMessage, BungusMessageChunk};
use crate::error::BungusError;

#[derive(Debug, sqlx::FromRow)]
pub struct BungusDatabase {
    pool: Option<SqlitePool>
}

impl BungusDatabase {
    pub async fn connect(path: PathBuf) -> Result<BungusDatabase, BungusError> {
        let mut database = BungusDatabase { pool: None };

        if !path.exists() {
            database.pool = Some(SqlitePool::connect(
                &format!("sqlite:{}", path.as_os_str().to_str().unwrap())
            ).await?);

            return Ok(database.run_migrations().await?);
        }

        database.pool = Some(SqlitePool::connect(
            &format!("sqlite:{}", path.as_os_str().to_str().unwrap())
        ).await?);

        Ok(database)
    }

    async fn run_migrations(self) -> Result<Self, BungusError> {
        if let Some(pool) = &self.pool {
            sqlx::migrate!("./migrations").run(pool).await?;
        }

        Ok(self)
    }

    pub async fn init_tokens(&mut self, server: GuildId) -> Result<(), BungusError> {
        todo!()
    }

    pub async fn set_token_weight(&mut self, root: usize, token: usize, weight: usize) -> Result<(), BungusError> {
        todo!()
    }

    pub async fn insert_server(&mut self, server: Guild) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                "INSERT INTO servers (server_id) VALUES (?) RETURNING id"
            ).bind(server.id.get() as i64).execute(pool).await?;
        }

        Ok(())
    }

    pub async fn delete_server(&mut self, server: Guild) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            let tx = pool.begin().await?;

            sqlx::query(
                "DELETE FROM servers WHERE server_id = ?"
            ).bind(server.id.get() as i64).execute(pool).await?;

            sqlx::query(
                "DELETE FROM channels WHERE server_id = ?"
            ).bind(server.id.get() as i64).execute(pool).await?;
            
            sqlx::query(
                "DELETE FROM messages WHERE server_id = ?"
            ).bind(server.id.get() as i64).execute(pool).await?;
            
            sqlx::query(
                "DELETE FROM tokens WHERE server_id = ?"
            ).bind(server.id.get() as i64).execute(pool).await?;

            tx.commit().await?;
        }

        Ok(())
    }

    pub async fn insert_channel(&mut self, channel: Channel) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                "INSERT INTO channels (channel_id, server_id) VALUES (?, ?) RETURNING id"
            ).bind(channel.id().get() as i64)
                .bind(channel.guild().unwrap().guild_id.get() as i64)
                .execute(pool).await?;
        }

        Ok(())
    }

    pub async fn insert_channels(&mut self, channel: Vec<Channel>) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            for c in channel {
            self.insert_channel(c).await?;
            }
        }

        Ok(())
    }

    pub async fn delete_channel(&mut self, channel: Channel) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            let tx = pool.begin().await?;

            sqlx::query(
                "DELETE FROM channels WHERE channel_id = ?"
            ).bind(channel.id().get() as i64).execute(pool).await?;

            sqlx::query(
                "DELETE FROM messages WHERE channel_id = ?"
            ).bind(channel.id().get() as i64).execute(pool).await?;

            tx.commit().await?;
        }

        Ok(())
    }

    pub async fn insert_message(&mut self, message: BungusMessage) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                "INSERT INTO messages (content, message_id, channel_id) VALUES (?, ?, ?) RETURNING id"
            ).bind(message.content)
                .bind(message.id.get() as i64)
                .bind(message.channel_id.get() as i64).execute(pool).await?;
        }

        Ok(())
    }

    pub async fn delete_message(&mut self, message_id: u64) -> Result<(), BungusError> {
        if let Some(pool) = &self.pool {
            sqlx::query(
                "DELETE * FROM messages WHERE message_id = ?"
            ).bind(message_id as i64).execute(pool).await?;
        }

        Ok(())
    }

    pub async fn insert_message_chunk(&mut self, message_chunk: BungusMessageChunk) -> Result<(), BungusError> {
        todo!()
    }

    pub async fn delete_message_chunk(&mut self, message_chunk: BungusMessageChunk) -> Result<(), BungusError> {
        todo!()
    }
}
