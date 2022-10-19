use std::{convert::Infallible, time::Duration};

use log::{error, trace};
use serenity::{
    async_trait,
    model::prelude::{ChannelType, Message},
};

use super::MessageReactor;

const TRADME_LINK_STATE: &str = "https://www.trademe.co.nz/a/property/residential/";

pub struct TrademeDistance;

impl TryFrom<Message> for TrademeDistance {
    type Error = Infallible;
    fn try_from(_: Message) -> Result<Self, Self::Error> {
        Ok(TrademeDistance)
    }
}

#[async_trait]
impl MessageReactor for TrademeDistance {
    fn name() -> &'static str {
        "trademe-distance"
    }

    fn description() -> &'static str {
        "A simple filter to react and create new threads whenever a trademe property link is sent"
    }

    fn precheck(message: &Message) -> bool {
        true
    }

    async fn process(
        self,
        message: &Message,
        app_state: &crate::state::AppState,
        ctx: &serenity::prelude::Context,
    ) {
        let content = &message.content;
        if content.contains(TRADME_LINK_STATE) {
            let channel_id = message.channel_id;

            let new_channel = match channel_id
                .create_public_thread(ctx, message.id, |f| {
                    f.kind(ChannelType::PublicThread)
                    // .auto_archive_duration(10080)
                })
                .await
            {
                Ok(c) => c,
                Err(e) => {
                    error!("failed to create new thread in response to trademe message due to error {:?}", e);
                    return;
                }
            };

            let typing = new_channel.start_typing(&ctx.http).unwrap();

            // try to parse out the full link if possible
            let links = message
                .content
                .split(" ")
                .find(|p| p.contains(TRADME_LINK_STATE));

            if links.is_none() {
                trace!("stopped trying to parse trademe link - as we were unable to find it");
                return;
            }
            let links = links.unwrap();

            let (tx, rx) = tokio::sync::oneshot::channel();

            app_state.tradme_api().add_to_queue(links, tx).await;

            // wait for api response
            //TODO

            let address: &str = ""; //TODO

            let (tx, rx) = tokio::sync::one::channel();
            app_state
                .maps_api()
                .add_to_queue(origin, destinations, return_channel);

            //wait for maps api
            //TODO

            let msg = new_channel.send_message(&ctx, |m| {}).await;

            if let Err(e) = msg {
                error!(
                    "failed to send trademe distance message to application due to error {:?}",
                    e
                );
            }

            typing.stop();
        }
    }
}
