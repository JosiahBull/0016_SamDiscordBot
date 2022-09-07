//! This module describes how interactions with the discord api should be handled initially
//! it recieves events from the discord WS and reacts to them accordingly.

//TODO: ideally we'd want to handle people joining/leaving guilds, so that we can either purge their roles when they leave
// or readd them if they happen to rejoin later down the line.

use log::{error, info, warn};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        guild::{Guild, Member, UnavailableGuild}, prelude::interaction::Interaction,
    },
};

use serenity::utils::read_image;

use crate::{
    discord_bot::{
        guilds::GuildHandler,
    }, AppState,
};

use super::{utils::BotDiscordId, manager::{InternalSender, DiscordEvent}};

#[allow(clippy::missing_docs_in_private_items)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let reader = ctx.data.read().await;

        let internal_sender = match reader.get::<InternalSender>() {
            Some(internal_sender) => internal_sender,
            None => {
                error!("InternalSender not found in context");
                return;
            }
        };

        if let Err(e) = internal_sender.send(DiscordEvent::Interaction(Box::new(interaction))) {
            error!("Error sending interaction to internal sender: {:?}", e);
        }
    }

    async fn guild_member_addition(&self, _ctx: Context, _new_member: Member) {
        warn!("New member joined, handler function not yet implemented");
        // todo!() //TODO: use this to readd a users roles if they have previously been verified
    }

    /// initalise a guild handler when the bot is added to a new guild
    async fn guild_create(&self, ctx: Context, guild: Guild, _: bool) {
        let data_read = ctx.data.read().await;

        //wait for botdiscordid to exist in the context
        while data_read.get::<BotDiscordId>().is_none() {
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }

        let id = match data_read.get::<BotDiscordId>() {
            Some(id) => id.get(),
            None => {
                error!("BotDiscordId not found in context");
                return;
            }
        };

        let internal_sender = match data_read.get::<InternalSender>() {
            Some(internal_sender) => internal_sender.clone(),
            None => {
                error!("InternalSender not found in context");
                return;
            }
        };

        let app_state = match data_read.get::<AppState>() {
            Some(app_state) => app_state.clone(),
            None => {
                error!("AppState not found in context");
                return;
            }
        };

        let mut guild_handler = GuildHandler::new(
            guild.id,
            guild.name,
            ctx.clone(),
            app_state,
            id,
            internal_sender.clone(),
        );
        guild_handler.start();

        if let Err(e) = internal_sender.send(DiscordEvent::NewGuild(guild_handler)) {
            error!("Error sending new guild to internal sender: {:?}", e);
        }
    }

    /// notify the global handler that a guild has been deleted and can no longer be monitored
    /// this is so we don't have old handlers accumulating.
    async fn guild_delete(&self, ctx: Context, guild: UnavailableGuild, _: Option<Guild>) {
        let data_read = ctx.data.read().await;

        let internal_sender = match data_read.get::<InternalSender>() {
            Some(internal_sender) => internal_sender,
            None => {
                error!("InternalSender not found in context");
                return;
            }
        };

        if let Err(e) = internal_sender.send(DiscordEvent::DeletedGuild(guild.id.0)) {
            error!("Error sending deleted guild to internal sender: {:?}", e);
        }
    }

    #[allow(unused_mut)]
    async fn ready(&self, ctx: Context, mut ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // set bot id for global state
        {
            let mut data_write = ctx.data.write().await;
            data_write.insert::<BotDiscordId>(BotDiscordId::new(ready.user.id.0));
        }

        // NOTE: it's easy to hit rate limits, so we need to be careful about this
        let image = match read_image("assets/profile.jpg") {
            Ok(image) => image,
            Err(e) => {
                error!("Error reading logo image: {:?}", e);
                return;
            }
        };

        ready
            .user
            .edit(&ctx, |p| {
                p.avatar(Some(&image))
                    .email("info@josiahbull.com")
                    .username("The NPC")
            })
            .await
            .unwrap_or_else(|_| error!("unable to set profile picture"));
    }
}
