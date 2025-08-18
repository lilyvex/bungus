use poise::serenity_prelude::{ChannelId, GuildChannel, GuildId, Http};

use crate::Context;
use crate::error::BungusError;

pub async fn index_channels(server_id: GuildId) -> Result<Vec<GuildChannel>, BungusError> {
    let http = Http::new(&std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set"));

    Ok(http.get_channels(server_id).await?)
}

// TODO: Continue collecting message chunks and place them into an SQLite database.
async fn scan_channel_messages(ctx: Context<'_>, channel: ChannelId) -> Result<(), BungusError> {
    Ok(())
}