use serenity::{async_trait, builder::{CreateApplicationCommand, CreateInteractionResponse}, model::prelude::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}, prelude::Context};

use crate::AppState;

use super::{command::Command, util::CommandResponse};

pub struct PingCommand;

impl<'a> TryFrom<&'a ApplicationCommandInteraction> for PingCommand {
    type Error = String;
    fn try_from(_: &'a ApplicationCommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[async_trait]
impl<'a> Command<'a> for PingCommand {
    fn name() -> &'static str {
        "ping"
    }

    fn description() -> &'static str {
        "Pings the bot, expect a pong response."
    }

    fn get_application_command_options(_: &mut CreateApplicationCommand) { }

    async fn handle_application_command<'b> (
        self,
        _: &'b ApplicationCommandInteraction,
        _: &'b AppState,
        _: &'b Context,
    ) -> Result<CommandResponse<'b>, CommandResponse<'b>> {
        Ok(CommandResponse::ComplexSuccess(
            CreateInteractionResponse::default()
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| data.content("Pong!").ephemeral(true))
            .to_owned()
        ))
    }
}