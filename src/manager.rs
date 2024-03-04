//! The global manager for the bot, which manages all guilds as individual tasks
//! and coordinates events between them.

use std::{collections::HashMap, ops::DerefMut, time::Duration};

use log::{error, warn};
use serenity::{
    all::Interaction,
    futures::{stream::FuturesUnordered, StreamExt},
    model::prelude::Message,
    prelude::GatewayIntents,
    Client,
};
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};

use super::{guilds::GuildHandler, handler::Handler};

/// An event that may occur between the various discord services
#[derive(Debug)]
pub enum DiscordEvent {
    /// a guild has been added, and must be managed
    NewGuild(GuildHandler),
    /// a guild was deleted and should no longer be managed
    DeletedGuild(u64),
    /// an interaction has been received from the user, and must be handled by a specific guild
    // enum variant boxed, as is quite large and so should be heap-allocated
    Interaction(Box<Interaction>),
    /// a new message received from any guild
    Message(Box<Message>),
    /// a shutdown command to be sent to a guild, when received the guild should cease all activity and shut down
    Shutdown,
}

/// A channel that can be used to send messages between guild handlers and the master discord process
#[derive(Clone)]
pub struct InternalSender(UnboundedSender<DiscordEvent>);

impl std::ops::Deref for InternalSender {
    type Target = UnboundedSender<DiscordEvent>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InternalSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A builder for the discord bot
pub struct DiscordBotBuilder<T> {
    /// the discord token to use for authentication with the discord api
    discord_token: Option<String>,
    /// the database to use for storing data
    app_state: Option<T>,
}

impl<T> DiscordBotBuilder<T> {
    /// set the token for the discord bot
    pub fn discord_token(mut self, token: String) -> Self {
        self.discord_token = Some(token);
        self
    }

    /// The application state for the bot. This is required for the bot to
    /// communicate with a database.
    pub fn state(mut self, app_state: T) -> Self {
        self.app_state = Some(app_state);
        self
    }

    /// Build the bot, and create a [DiscordBot] instance.
    pub fn build(self) -> Result<DiscordBot<T>, String> {
        let discord_token = match self.discord_token {
            Some(token) => token,
            None => return Err("No token provided".to_string()),
        };

        let app_state = match self.app_state {
            Some(app_state) => app_state,
            None => return Err("No app state provided".to_string()),
        };

        Ok(DiscordBot {
            discord_token,
            app_state,
        })
    }
}

impl<T> Default for DiscordBotBuilder<T> {
    fn default() -> Self {
        DiscordBotBuilder {
            discord_token: None,
            app_state: None,
        }
    }
}

/// A discord bot global manager, runs asynchronously communicating via channels
/// supports graceful and force shutdown, among many other thingss
pub struct DiscordBot<T> {
    /// the discord token to use for authentication with the discord api
    discord_token: String,
    /// the database to use for storing data
    app_state: T,
}

impl<T: Send + Sync + 'static + Clone> DiscordBot<T> {
    /// Get a builder to setup a new discord bot
    pub fn builder() -> DiscordBotBuilder<T> {
        DiscordBotBuilder::default()
    }

    /// Start the discord bot, this will connect to discords api and create internal
    /// handlers as required.
    /// Will exit when the bot has fully disconnected from all services.
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> { // TODO: update this to be "spawn_discord_task", it should return a cancel token. We also need to setup a heartbeat.
        let intents = GatewayIntents::all();

        let mut client = Client::builder(&self.discord_token, intents)
            .event_handler(Handler)
            .await?;

        client.start().await?;

        Ok(())
    }
}
