mod trademe;

use crate::{discord_bot::messages::trademe::TrademeDistance, state::AppState};
use serenity::{async_trait, model::prelude::Message, prelude::Context};

#[async_trait]
trait MessageReactor<'a>: TryFrom<&'a Message> {
    /// Get the name of the command
    fn name() -> &'static str;

    /// Get the description of the command
    fn description() -> &'static str;

    /// Check if this message matches the precondition to attempt parsing this command
    fn precheck(message: &Message) -> bool;

    /// Attempt to parse this message and take an action in response
    async fn process(self, message: &Message, app_state: &AppState, ctx: &Context);
}

macro_rules! reactor {
    ( $cmd:expr, $state:expr, $context:expr, $( $x:ty ),* $(,)? )  => {
        {
            /// ensures that the provided type has relevant traits
            fn _ensure_traits<'a, T: MessageReactor<'a, Error=String>>() {}
            $(
                _ensure_traits::<$x>();

                if <$x>::precheck($cmd) {
                    if let Ok(v_cmd) = <$x>::try_from($cmd) {
                        v_cmd.process($cmd, $state, $context).await;
                    }
                }
            )*
        }
    };
}

pub async fn non_command_message(
    message: &Message,
    app_state: &AppState,
    ctx: &Context,
) -> Result<(), String> {
    reactor!(message, app_state, ctx, TrademeDistance);

    Ok(())
}
