use crate::Context;
use crate::error::BungusError;

#[poise::command(slash_command, prefix_command)]
pub async fn markov(
    ctx: Context<'_>,
    #[description = "Markov chain prompt"] prompt: String,
) -> Result<(), BungusError> {
    ctx.say("Markov functionality is under construction!").await?;
    Ok(())
}