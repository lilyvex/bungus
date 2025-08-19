use crate::Context;
use crate::error::BungusError;
use crate::markov::model::MODEL;

#[poise::command(slash_command, prefix_command)]
pub async fn markov(
    ctx: Context<'_>,
    #[description = "Markov chain prompt"] _prompt: String,
) -> Result<(), BungusError> {
    let output = {
        let model = MODEL.read().await;
        model.generate()
    };

    if output.is_none() {
        ctx.say("> Bungus couldn't think of what to say!").await?;

        return Ok(());
    }

    ctx.say(output.unwrap()).await?;
    Ok(())
}