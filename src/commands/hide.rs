use std::convert::Infallible;

use serenity::{async_trait, builder::CreateApplicationCommand, model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};

use crate::AppState;

use super::{command::Command, util::CommandResponse};

pub struct Hide;

impl<'a> TryFrom<&'a ApplicationCommandInteraction> for Hide {
    type Error = Infallible;
    fn try_from(_: &'a ApplicationCommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[async_trait]
impl<'a> Command<'a> for Hide {
    fn name(&self) -> &'static str {
        "hide"
    }

    fn description(&self) -> &'static str {
        "Creates a large message to hide previous messages in the chat"
    }

    fn get_application_command_options(_: &mut CreateApplicationCommand) { }

    #[allow(clippy::invisible_characters)]
    async fn handle_application_command<'b> (
        self,
        _: &'b ApplicationCommandInteraction,
        _: &'b AppState,
        _: &'b Context,
    ) -> Result<CommandResponse<'b>, CommandResponse<'b>> {
        Ok(CommandResponse::BasicSuccess(String::from("​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​​\n​​\n​​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​​\n​​\n​​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n​\n")))
    }
}