use poise::{serenity_prelude as serenity, CreateReply};
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use crate::Context;
use crate::error::BungusError;

#[poise::command(slash_command, prefix_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), BungusError> {
    let profile_image = ctx.cache().current_user().avatar_url().unwrap().clone();
    ctx.send(CreateReply::default().embed(
        CreateEmbed::new()
            .title("🛠️ About Bungus")
            .description("Bungus is an open-source (GPLv3) Markov chain Discord bot.\n\
                          It scans messages from channels in your server to build a \n\
                          Markov chain, allowing it to create random and sometimes  \n\
                          hilarious sentences.                                      \n\
                          \n\
                          Bungus can be prompted with the `/markov` command to invoke \n\
                          a response. It will also respond randomly, using the replied\n\
                          to message as the prompt.\n\
                          \n\
                          [Repository](https://github.com/lilyvex/bungus)")
            .color(0x7A69FF)
            .thumbnail(profile_image)
            .footer(CreateEmbedFooter::new("Made with ❤️ by Lily"))
    )).await?;

    Ok(())
}