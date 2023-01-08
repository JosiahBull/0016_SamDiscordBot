mod discord_bot;
mod google_api;
mod trademe_api;

mod healthcheck;

mod logging;
mod state;

use log::{error, info};
use std::process::exit;

use crate::{
    discord_bot::DiscordBot, google_api::maps::GoogleMapsApi, logging::configure_logger,
    state::AppState, trademe_api::TrademeApi,
};

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
    let google_maps_token =
        std::env::var("GOOGLE_MAPS_TOKEN").expect("GOOGLE_MAPS_TOKEN must be set");
    let geckodriver_url = std::env::var("GECKO_DRIVER").expect("GECKO_DRIVER must be set");

    info!("spawning google maps handler");
    let mut google_maps_api_handler = GoogleMapsApi::builder().key(google_maps_token).build();
    let google_maps_api_handle = google_maps_api_handler.handle();
    let google_maps_thread_handle = tokio::spawn(async move {
        google_maps_api_handler.run().await;
    });

    info!("spawning trademe handler");
    let mut trademe_api_handler = TrademeApi::builder()
        .gecko_driver_url(geckodriver_url)
        .build()
        .await;
    let trademe_api_handle = trademe_api_handler.handle();
    let trademe_thread_handle = tokio::spawn(async move {
        trademe_api_handler.run().await;
    });

    let state = AppState::new(database_url, google_maps_api_handle, trademe_api_handle).await?;

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

    info!("creating healthcheck server");
    let healthcheck_state = state.clone();
    let healthcheck_handle = tokio::task::spawn(async move {
        let builder = healthcheck::Healthcheck::builder()
            .state(healthcheck_state)
            .build();

        let mut server = match builder.await {
            Ok(server) => server,
            Err(e) => {
                error!("failed to build healthcheck server: {}", e);
                return;
            }
        };

        server.run().await;

        info!("healthcheck server shut down");
    });

    tokio::pin!(discord_handle);
    tokio::pin!(google_maps_thread_handle);
    tokio::pin!(trademe_thread_handle);
    tokio::pin!(healthcheck_handle);

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

            _ = google_maps_thread_handle => {
                info!("google maps handler shut down");
                break;
            }

            _ = trademe_thread_handle => {
                info!("trademe handler shut down");
                break;
            }

            _ = healthcheck_handle => {
                info!("healthcheck server shut down");
                break;
            }
        }
    }

    info!("global TomBot shutdown");

    Ok(())
}
