use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};

use crate::state::AppState;

use super::{command::Command, util::CommandResponse};

pub struct HideCommand;

impl<'a> TryFrom<&'a ApplicationCommandInteraction> for HideCommand {
    type Error = String;
    fn try_from(_: &'a ApplicationCommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[async_trait]
impl<'a> Command<'a> for HideCommand {
    fn name() -> &'static str {
        "hide"
    }

    fn description() -> &'static str {
        "Creates a large message to hide previous messages in the chat"
    }

    fn get_application_command_options(_: &mut CreateApplicationCommand) {}

    #[allow(clippy::invisible_characters)]
    async fn handle_application_command<'b>(
        self,
        _: &'b ApplicationCommandInteraction,
        _: &'b AppState,
        _: &'b Context,
    ) -> Result<CommandResponse<'b>, CommandResponse<'b>> {
        Ok(CommandResponse::ComplexSuccess(
            CreateInteractionResponse::default()
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| data.content("​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​​\n​​\n​​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​​\n​​\n​​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n"))
            .to_owned()
        ))
    }
}
