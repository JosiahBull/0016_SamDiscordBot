use std::{
    error::Error,
    sync::{Arc, RwLock},
    time::Duration,
};

use lazy_static::lazy_static;
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::Deserialize;
use serenity::prelude::TypeMapKey;

use crate::{google_api::maps::GoogleMapsApiHandle, trademe_api::TrademeApiHandle};

#[derive(Deserialize)]
pub struct TomlConfig {
    pub head_tennant_acc_number: String,
    pub destinations: Vec<Destination>,
    pub flatmates: Vec<Flatmate>,
    pub phrases: Vec<String>,
    pub powered_by: Vec<String>,
}

#[derive(Deserialize)]
pub struct Flatmate {
    pub discord_id: u64,
    pub name: String,
    pub display_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Destination {
    pub label: String,
    pub address: String,
}

lazy_static! {
    pub static ref CONFIG: TomlConfig = {
        let config = std::fs::read_to_string("config.toml").expect("Failed to read config.toml");
        toml::from_str(&config).expect("Failed to parse config.toml")
    };
}

/// A connection to the database, representing the stored "state" of the app
pub struct AppState {
    pub google_api: Arc<RwLock<GoogleMapsApiHandle>>,
    pub trademe_api: Arc<RwLock<TrademeApiHandle>>,

    pub database: Arc<DatabaseConnection>,
}

impl AppState {
    pub async fn new(
        database_url: String,
        google_api: GoogleMapsApiHandle,
        trademe_api: TrademeApiHandle,
    ) -> Result<Self, Box<dyn Error>> {
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        let connection = Database::connect(opt).await?;

        info!("starting database migration...");
        Migrator::up(&connection, None).await?;
        info!("migration complete");

        // load CONFIG lazy_static here
        info!("loading config...");
        let _ = *CONFIG; //IDK if this will load it
        info!("config loaded");

        Ok(Self {
            google_api: Arc::new(RwLock::new(google_api)),
            trademe_api: Arc::new(RwLock::new(trademe_api)),

            database: Arc::new(connection),
        })
    }

    pub fn maps_api(&self) -> GoogleMapsApiHandle {
        self.google_api.read().unwrap().clone()
    }

    pub fn trademe_api(&self) -> TrademeApiHandle {
        self.trademe_api.read().unwrap().clone()
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState").finish()
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            google_api: self.google_api.clone(),
            trademe_api: self.trademe_api.clone(),

            database: self.database.clone(),
        }
    }
}

impl TypeMapKey for AppState {
    type Value = AppState;
}
