//! This module describes how interactions with the discord api should be handled initially
//! it receives events from the discord WS and reacts to them accordingly.

use log::{error, info, warn};
use serenity::{
    all::{GuildId, Interaction},
    async_trait,
    builder::{CreateAttachment, EditProfile},
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        guild::{Guild, Member, UnavailableGuild},
        prelude::Message,
    },
};

use super::manager::{DiscordEvent, InternalSender};


// let handle = tokio::task::spawn(async move {
//     let mut thread_handles = FuturesUnordered::new();
//     let mut guild_handlers: HashMap<u64, GuildHandler> = HashMap::default();

//     loop {
//         select! {
//             Some(i_e) = i_rx.recv() => {
//                 match i_e {
//                     DiscordEvent::NewGuild(handler) => {
//                         // finish creating the handler
//                         let key: u64 = handler.guild_id.into();
//                         if guild_handlers.contains_key(&key) {
//                             if let Err(e) = handler.close(Duration::from_secs(0)).await {
//                                 error!("failed to close a guild handler {}", e);
//                             }
//                         }
//                         guild_handlers.insert(key, handler);
//                     },
//                     DiscordEvent::DeletedGuild(guild) => {
//                         // remove guild handler
//                         let mut g_h = match guild_handlers.remove(&guild) {
//                             Some(s) => s,
//                             None => {
//                                 error!("Tried to remove non-existant guild.");
//                                 return;
//                             }
//                         };

//                         let t_h = tokio::task::spawn(async move {
//                             if let Err(e) = g_h.close(Duration::from_secs(5)).await {
//                                 error!("Failed to close a guild handler '{}'.", e);
//                             }
//                         });

//                         thread_handles.push(t_h);
//                     },
//                     DiscordEvent::Interaction(interaction) => {
//                         let guild_id: Option<serenity::model::id::GuildId> = match *interaction {
//                             Interaction::Ping(_) => {
//                                 error!("Ignored ping application command.");
//                                 continue;
//                             },
//                             Interaction::Command(ref c) => c.guild_id,
//                             Interaction::Component(ref c) => c.guild_id,
//                             Interaction::Autocomplete(ref c) => c.guild_id,
//                             Interaction::Modal(ref c) => c.guild_id,
//                             _ => todo!("Currently, all branches are covered - more may become available in the future!"),
//                         };

//                         let guild_id: u64 = match guild_id {
//                             Some(g_id) => g_id.into(),
//                             None => {
//                                 error!("Got interaction without guild id.");
//                                 continue;
//                             }
//                         };

//                         let g_h = match guild_handlers.get(&guild_id) {
//                             Some(s) => s.internal_tx.clone(),
//                             None => {
//                                 error!("Tried to handle message for non-existant guild id '{}'.", guild_id);
//                                 return;
//                             }
//                         };
//                         if let Err(e) = g_h.send(DiscordEvent::Interaction (interaction)) {
//                             error!("Failed to send interaction to guild handler '{}'.", e);
//                         }
//                     },
//                     DiscordEvent::Message(message) => {
//                         let guild_id: u64 = match message.guild_id {
//                             Some(g_id) => g_id.into(),
//                             None => {
//                                 warn!("got message without guild id");
//                                 continue;
//                             }
//                         };

//                         let g_h = match guild_handlers.get(&guild_id) {
//                             Some(s) => s.internal_tx.clone(),
//                             None => {
//                                 error!("tried to handle message for non-existant guild id {}", guild_id);
//                                 return;
//                             }
//                         };

//                         if let Err(e) = g_h.send(DiscordEvent::Message(message)) {
//                             error!("failed to send message to guild handler {}", e);
//                         }
//                     }
//                     e => error!("unexpected discord event received {:?}", e),
//                 }
//             },
//             _ = thread_handles.next(), if !thread_handles.is_empty() => {} //drain the handles as they complete
//             else => {
//                 panic!("both receivers closed without breaking the loop, this indicates a failure")
//             }
//         }
//     }
// });

#[allow(clippy::missing_docs_in_private_items)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: &Context, interaction: &Interaction) {
        let guild_id: Option<GuildId> = match *interaction {
            Interaction::Ping(_) => {
                // XXX: handle this
                error!("Ignored ping application command.");
                return;
            },
            Interaction::Command(ref c) => c.guild_id,
            Interaction::Component(ref c) => c.guild_id,
            Interaction::Autocomplete(ref c) => c.guild_id,
            Interaction::Modal(ref c) => c.guild_id,
            _ => {
                error!("The discord API has changed, and this bot has not been updated to handle the new interaction type.");
                return;
            },
        };

        let guild_id: u64 = match guild_id {
            Some(g_id) => g_id.into(),
            None => {
                error!("Got interaction without guild id.");
                return;
            }
        };

        // let g_h = match guild_handlers.get(&guild_id) {
        //     Some(s) => s.internal_tx.clone(),
        //     None => {
        //         error!("Tried to handle message for non-existant guild id '{}'.", guild_id);
        //         return;
        //     }
        // };
        // if let Err(e) = g_h.send(DiscordEvent::Interaction (interaction)) {
        //     error!("Failed to send interaction to guild handler '{}'.", e);
        // }
    }

    async fn message(&self, ctx: &Context, message: &Message) {
        // let reader = ctx.data.read().await;

        // let internal_sender = match reader.get::<InternalSender>() {
        //     Some(internal_sender) => internal_sender,
        //     None => {
        //         error!("InternalSender not found in context");
        //         return;
        //     }
        // };

        // if let Err(e) = internal_sender.send(DiscordEvent::Message(Box::new(*message))) {
        //     error!("Error sending message to internal sender: {:?}", e);
        // }
    }

    async fn guild_member_addition(&self, _ctx: &Context, _new_member: &Member) {
        warn!("New member joined, handler function not yet implemented");
        // todo!() //TODO: use this to readd a users roles if they have previously been verified
    }

    /// initalise a guild handler when the bot is added to a new guild
    async fn guild_create(&self, ctx: &Context, guild: &Guild, _: &Option<bool>) {
        // let data_read = ctx.data.read().await;

        // //wait for the bot's discord id to exist in the context
        // while data_read.get::<BotDiscordId>().is_none() {
        //     tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        // }

        // let id = match data_read.get::<BotDiscordId>() {
        //     Some(id) => id.get(),
        //     None => {
        //         error!("BotDiscordId not found in context");
        //         return;
        //     }
        // };

        // let internal_sender = match data_read.get::<InternalSender>() {
        //     Some(internal_sender) => internal_sender.clone(),
        //     None => {
        //         error!("InternalSender not found in context");
        //         return;
        //     }
        // };

        // let app_state = match data_read.get::<AppState>() {
        //     Some(app_state) => app_state.clone(),
        //     None => {
        //         error!("AppState not found in context");
        //         return;
        //     }
        // };

        // let mut guild_handler = GuildHandler::new(
        //     guild.id,
        //     guild.name,
        //     ctx.clone(),
        //     app_state,
        //     id,
        //     internal_sender.clone(),
        // );
        // guild_handler.start();

        // if let Err(e) = internal_sender.send(DiscordEvent::NewGuild(guild_handler)) {
        //     error!("Error sending new guild to internal sender: {:?}", e);
        // }
    }

    /// notify the global handler that a guild has been deleted and can no longer be monitored
    /// this is so we don't have old handlers accumulating.
    async fn guild_delete(&self, ctx: &Context, guild: &UnavailableGuild, _: &Option<Guild>) {
        // let data_read = ctx.data.read().await;

        // let internal_sender = match data_read.get::<InternalSender>() {
        //     Some(internal_sender) => internal_sender,
        //     None => {
        //         error!("InternalSender not found in context");
        //         return;
        //     }
        // };

        // if let Err(e) = internal_sender.send(DiscordEvent::DeletedGuild(guild.id.0.into())) {
        //     error!("Error sending deleted guild to internal sender: {:?}", e);
        // }
    }

    #[allow(unused_mut)]
    async fn ready(&self, ctx: &Context, mut ready: &Ready) {
        // info!("{} is connected!", ready.user.name);

        // // set bot id for global state
        // {
        //     let mut data_write = ctx.data.write().await;
        //     data_write.insert::<BotDiscordId>(BotDiscordId::new(ready.user.id.0.into()));
        // }

        // ready
        //     .user
        //     .edit(
        //         &ctx,
        //         EditProfile::new()
        //             .avatar(
        //                 &CreateAttachment::path("./assets/profile.jpg")
        //                     .await
        //                     .unwrap(),
        //             )
        //             .username("The NPC"),
        //     )
        //     .await
        //     .unwrap_or_else(|_| error!("unable to set profile picture"));
    }
}
