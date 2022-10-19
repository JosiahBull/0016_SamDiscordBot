mod database;
mod discord_bot;
mod google_api;
mod trademe_api;

mod logging;
mod state;

use dotenv::dotenv;
use log::{error, info};
use std::process::exit;

use crate::{
    discord_bot::DiscordBot, google_api::maps::GoogleMapsApi, logging::configure_logger,
    state::AppState, trademe_api::TrademeApi,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_logger()?;

    dotenv().ok();

    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let google_maps_token =
        std::env::var("GOOGLE_MAPS_TOKEN").expect("GOOGLE_MAPS_TOKEN must be set");
    let geckodriver_url = std::env::var("GECKO_DRIVER").expect("GECKO_DRIVER must be set");

    let state = AppState::new();

    info!("spawning google maps handler");
    let mut google_maps_state = state.clone();
    let google_maps_handler = tokio::spawn(async move {
        let mut google_maps_handler = GoogleMapsApi::builder().key(google_maps_token).build();

        google_maps_state.set_google_api(google_maps_handler.handle());

        google_maps_handler.run().await;
    });

    info!("spawning trademe handler");
    let mut tradme_state = state.clone();
    let trademe_handler = tokio::spawn(async move {
        let mut trademe_handler = TrademeApi::builder()
            .gecko_driver_url(geckodriver_url)
            .build()
            .await;
        tradme_state.set_tradme_api(trademe_handler.handle());

        trademe_handler.run().await;
    });

    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db_handle = database::DatabaseHandle::connect(db_url).await?;

    info!("spawning discord handler");
    let discord_state = state.clone();
    let discord_handle = tokio::task::spawn(async move {
        let builder = DiscordBot::builder()
            .discord_token(discord_token)
            .state(discord_state)
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
    tokio::pin!(google_maps_handler);

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

            _ = google_maps_handler => {
                info!("google maps handler shut down");
                break;
            }

            _ = trademe_handler => {
                info!("trademe handler shut down");
                break;
            }
        }
    }

    info!("global TomBot shutdown");

    Ok(())
}
