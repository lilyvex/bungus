use poise::serenity_prelude::{ChannelId, GuildId, MessageId};
use poise::serenity_prelude::builder::GetMessages;

use crate::Context;
use crate::error::BungusError;

pub struct BungusMessage {
    pub id: MessageId,
    pub server_id: GuildId,
    pub channel_id: ChannelId,
    pub content: String,
}

pub struct BungusMessageChunk {
    messages: Vec<BungusMessage>,
}

async fn index_channels(ctx: Context<'_>) -> Result<(), BungusError> {
    todo!();

    Ok(())
}

// TODO: Continue collecting message chunks and place them into an SQLite database.
async fn scan_channel_messages(ctx: Context<'_>, channel: ChannelId) -> Result<(), BungusError> {
    let builder = GetMessages::new().limit(4);
    let messages = channel.messages(&ctx, builder).await?;
    let mut message_chunk = BungusMessageChunk { messages: vec![] };

    for message in messages {
        message_chunk.messages.push(
            BungusMessage {
                id: message.id,
                server_id: ctx.guild_id().unwrap(),
                channel_id: channel,
                content: message.content
            }
        );
    }

    Ok(())
}