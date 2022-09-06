mod discord_bot;

use dotenv::dotenv;
use log::{info, error};
use serenity::prelude::TypeMapKey;
use std::{fmt::Debug, process::exit};

use crate::discord_bot::DiscordBot;

/// A connection to the database, representing the stored "state" of the app
pub struct AppState {

}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {  }
    }
}

impl TypeMapKey for AppState {
    type Value = AppState;
}

fn configure_logger() -> Result<(), Box<dyn std::error::Error>> {
    // Configure logger at runtime
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %I:%M:%S %P]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("h2", log::LevelFilter::Info)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("tracing", log::LevelFilter::Warn)
        .level_for("serenity", log::LevelFilter::Warn)
        .level_for("reqwest", log::LevelFilter::Warn)
        .level_for("rustls", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_logger()?;

    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN")?;

    info!("spawning discord handler");
    let discord_handle = tokio::task::spawn(async move {
        let builder = DiscordBot::builder()
            .token(token)
            .state(AppState {  })
            .build();

        let bot = match builder {
            Ok(bot) => bot,
            Err(e) => {
                error!("failed to build discord bot: {}", e);
                return;
            }
        };

        if let Err(e) = bot.run().await {
            error!("failed to run discord bot: {}", e);
            exit(1);
        }

        info!("discord bot shut down");
    });

    tokio::pin!(discord_handle);

    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                info!("received ctrl-c, shutting down");
                break;
            }

            _ = &mut discord_handle => {
                info!("discord handler shut down");
                break;
            }
        }
    }

    info!("global TomBot shutdown");

    Ok(())
}
