mod commands;
mod messages;
mod common;
mod database;
mod guilds;
mod handler;
mod healthcheck;
mod logging;
mod manager;
mod state;

use log::{error, info};
use std::process::exit;

use crate::{logging::configure_logger, manager::DiscordBot, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_logger()?;

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        std::env::var("POSTGRES_PASS").expect("POSTGRES_PASSWORD must be set"),
        std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set"),
        std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set"),
        std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set")
    );
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let state = AppState::new(database_url).await?;

    info!("Spawning discord handler.");
    let discord_state = state.clone();
    let discord_handle = tokio::task::spawn(async move {
        let builder = DiscordBot::builder()
            .discord_token(discord_token)
            .state(discord_state)
            .build();

        let bot = match builder {
            Ok(bot) => bot,
            Err(e) => {
                error!("Failed to build discord bot: '{}'.", e);
                return;
            }
        };

        if let Err(e) = bot.run().await {
            error!("Failed to run discord bot: '{}'.", e);
            exit(1);
        }

        info!("Discord bot shut down.");
    });

    info!("creating healthcheck server");
    let healthcheck_state = state.clone();
    let healthcheck_handle = tokio::task::spawn(async move {
        let builder = healthcheck::Healthcheck::builder()
            .state(healthcheck_state)
            .build();

        let mut server = match builder.await {
            Ok(server) => server,
            Err(e) => {
                error!("Failed to build healthcheck server: '{}'.", e);
                return;
            }
        };

        server.run().await;

        info!("Healthcheck server shut down.");
    });

    tokio::pin!(discord_handle);
    tokio::pin!(healthcheck_handle);

    // loop {
    //     tokio::select! {
    //         biased;
    //         _ = tokio::signal::ctrl_c() => {
    //             info!("Received ctrl-c, shutting down.");
    //             break;
    //         }

    //         _ = &mut discord_handle => {
    //             info!("Discord handler shut down.");
    //             break;
    //         }

    //         _ = healthcheck_handle => {
    //             info!("Healthcheck server shut down.");
    //             break;
    //         }
    //     }
    // }

    info!("Global TomBot Shutdown.");

    Ok(())
}
