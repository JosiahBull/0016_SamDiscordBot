use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateInteractionResponse},
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};

use crate::state::AppState;

use super::{
    command::Command,
    util::{CommandResponse, FailureMessageKind},
};

pub struct SayCommand<'a> {
    message: &'a str,
}

impl<'a> TryFrom<&'a ApplicationCommandInteraction> for SayCommand<'a> {
    type Error = String;
    fn try_from(interaction: &'a ApplicationCommandInteraction) -> Result<Self, Self::Error> {
        let message = interaction
            .data
            .options
            .get(0)
            .ok_or("No message provided")?
            .value
            .as_ref()
            .ok_or("No message provided")?
            .as_str()
            .ok_or("No message provided")?;
        Ok(Self { message })
    }
}

#[async_trait]
impl<'a> Command<'a> for SayCommand<'a> {
    fn name() -> &'static str {
        "say"
    }

    fn description() -> &'static str {
        "Says whatever you want!"
    }

    fn get_application_command_options(i: &mut CreateApplicationCommand) {
        i.create_option(|o| {
            o.name("text")
                .description("What you want the bot to say")
                .required(true)
                .kind(CommandOptionType::String)
                .max_length(1900)
        });
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b ApplicationCommandInteraction,
        _: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse<'b>, CommandResponse<'b>> {
        if let Err(e) = interaction
            .channel_id
            .send_message(ctx, |m| m.content(self.message))
            .await
        {
            return Err(CommandResponse::ComplexFailure {
                response: String::from("Failed to use /say due to error"),
                kind: FailureMessageKind::Error,
                log_message: e.to_string(),
            });
        }

        Ok(CommandResponse::ComplexSuccess(
            CreateInteractionResponse::default()
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| {
                    data.content(format!("I will send: {}", self.message))
                        .ephemeral(true)
                })
                .to_owned(),
        ))
    }
}
