use crate::Context;
use crate::error::BungusError;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), BungusError> {
    ctx.say("Pong!").await?;
    Ok(())
}