mod trademe;

use crate::state::AppState;
use serenity::{async_trait, model::prelude::Message, prelude::Context};

#[async_trait]
trait MessageReactor: TryFrom<Message> {
    /// Get the name of the command
    fn name() -> &'static str;

    /// Get the description of the command
    fn description() -> &'static str;

    /// Check if this message matches the precondition to attempt parsing this command
    fn precheck(message: &Message) -> bool;

    /// Attempt to parse this message and take an action in response
    async fn process(self, message: &Message, app_state: &AppState, ctx: &Context);
}
