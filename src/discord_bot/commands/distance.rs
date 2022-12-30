use log::error;
use serenity::{
    all::{CommandInteraction, CommandOptionType},
    async_trait,
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage, EditInteractionResponse,
    },
    prelude::Context,
};

use crate::{discord_bot::common::distance::load_maps_data_to_embed, state::AppState};

use super::{
    command::Command,
    util::{CommandResponse, FailureMessageKind},
};

pub struct DistanceCommand;

impl<'a> TryFrom<&'a CommandInteraction> for DistanceCommand {
    type Error = String;
    fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[async_trait]
impl<'a> Command<'a> for DistanceCommand {
    fn name() -> &'static str {
        "distance"
    }

    fn description() -> &'static str {
        "calculate distances from here to major locations, in minutes - utilises the google maps api"
    }

    fn get_application_command_options(cmd: CreateCommand) -> CreateCommand {
        cmd.add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "address",
                "The address to show locations for",
            )
            .required(true)
            .max_length(200)
            .to_owned(),
        )
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        state: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        // create an "in progress" response
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()),
            )
            .await
            .map_err(|e| CommandResponse::ComplexFailure {
                response: String::from("Failed to create interaction response"),
                kind: FailureMessageKind::Error,
                log_message: format!("Failed to create interaction response: {}", e),
            })?;

        // parse the address
        let address = interaction.data.options.get(0).unwrap(); //shouldn't be possible to send without this parameter being set as its required
        let address = &address.value;
        let address: String = address.as_str().unwrap().to_string();

        let data = load_maps_data_to_embed(address.clone(), state).await;
        if let Err(e) = data {
            error!(
                "Failed to calculate distances for {} due to error {}",
                address, e
            );
            interaction
                .edit_response(
                    &ctx,
                    EditInteractionResponse::new()
                        .content("Google API returned error, it has been logged."),
                )
                .await
                .unwrap();

            return Ok(CommandResponse::NoResponse);
        }
        let data = data.unwrap();

        if let Err(e) = interaction
            .edit_response(&ctx, EditInteractionResponse::new().embed(data).content(""))
            .await
        {
            error!("Failed to return embed: {}", e);
        }

        Ok(CommandResponse::NoResponse) // we are handling the response ourselves
    }
}
