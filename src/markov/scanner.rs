use poise::serenity_prelude::{ChannelId, GuildChannel, GuildId, Http, MessageId};
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

async fn index_channels(ctx: Context<'_>, server_id: GuildId) -> Result<Vec<GuildChannel>, BungusError> {
    let http = Http::new(&std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"));

    Ok(http.get_channels(server_id).await?)
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