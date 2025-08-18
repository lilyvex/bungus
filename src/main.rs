extern crate dotenv;

mod error;
mod commands;
mod markov;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GuildId;

use log::info;

use crate::error::BungusError;

struct Data {}
type Context<'a> = poise::Context<'a, Data, BungusError>;

#[tokio::main]
async fn main() -> Result<(), BungusError> {
    pretty_env_logger::init_timed();
    info!("Initializing environment...");

    dotenv()?;
    info!("Environment initialized");

    let token: String = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let intents = serenity::GatewayIntents::non_privileged() & serenity::GatewayIntents::GUILD_MESSAGES;
    let framework: poise::Framework<Data, BungusError> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping::ping(),
                commands::markov::markov()
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                if std::env::var("GLOBALLY_REGISTER_COMMANDS") == Ok("true".to_string()) {
                    poise::builtins::register_globally(
                        ctx,
                        &framework.options().commands
                    ).await?;
                } else {
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        GuildId::from(
                            std::env::var("GUILD_ID").expect("GUILD_ID not set")
                                .parse::<u64>()?)).await?;
                }

                info!("Successfully registered commands");

                Ok(Data {})
            })
        })
        .build();

    info!("Starting client...");
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await?;

    Ok(())
}
