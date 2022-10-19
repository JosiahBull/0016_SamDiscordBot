use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};

use crate::{google_api::maps::GoogleMapsData, state::AppState};

use super::{
    command::Command,
    util::{CommandResponse, FailureMessageKind},
};

const DESTINATIONS: &[[&str; 2]] = &[
    ["UoA", "University of Auckland"],
    [
        "Massey",
        "Massey University East Precinct Albany Expressway, SH17, Albany, Auckland 0632",
    ],
    [
        "Zerojet",
        "5 Te Apunga Place, Mount Wellington, Auckland 1060",
    ],
    ["Crown", "65 Hugo Johnston Drive, Penrose, Auckland, 1061"],
];

pub struct DistanceCommand;

impl<'a> TryFrom<&'a ApplicationCommandInteraction> for DistanceCommand {
    type Error = String;
    fn try_from(_: &'a ApplicationCommandInteraction) -> Result<Self, Self::Error> {
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

    fn get_application_command_options(i: &mut CreateApplicationCommand) {
        i.create_option(|o| {
            o.name("address")
                .description("The address to show locations for")
                .required(true)
                .kind(CommandOptionType::String)
                .max_length(200)
        });
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b ApplicationCommandInteraction,
        state: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse<'b>, CommandResponse<'b>> {
        // create an "in progress" response
        interaction
            .create_interaction_response(&ctx, |f| {
                f.kind(InteractionResponseType::DeferredChannelMessageWithSource)
            })
            .await
            .map_err(|e| CommandResponse::ComplexFailure {
                response: String::from("Failed to create interaction response"),
                kind: FailureMessageKind::Error,
                log_message: format!("Failed to create interaction response: {}", e),
            })?;

        // parse the address
        let address = interaction
            .data
            .options
            .get(0)
            .ok_or(CommandResponse::ComplexFailure {
                response: String::from("Failed to get address"),
                kind: FailureMessageKind::Error,
                log_message: String::from("Failed to get address"),
            })?;

        let address = address.value.as_ref();

        let address: String = address.unwrap().as_str().unwrap().to_string();

        // create a oneshot channel to await the response
        let (tx, rx) = tokio::sync::oneshot::channel();

        // make a global request for the address
        state
            .maps_api()
            .add_to_queue(address, DESTINATIONS, tx)
            .await;

        // wait for the oneshot channel to return (maximum of 20 seconds)
        let data: GoogleMapsData = tokio::time::timeout(std::time::Duration::from_secs(20), rx)
            .await
            .map_err(|_| CommandResponse::ComplexFailure {
                response: String::from("Timed out waiting for response"),
                kind: FailureMessageKind::Error,
                log_message: String::from("Timed out waiting for response"),
            })?
            .map_err(|e| CommandResponse::ComplexFailure {
                response: String::from("Failed to get response"),
                kind: FailureMessageKind::Error,
                log_message: format!("Failed to get response: {}", e),
            })?
            .map_err(|e| CommandResponse::ComplexFailure {
                response: String::from("Failed to get response"),
                kind: FailureMessageKind::Error,
                log_message: format!("Failed to get response from api: {}", e),
            })?;

        interaction
            .edit_original_interaction_response(&ctx, |f| {
                f.embed(|e| {
                    e.title(&data.origin_addresses[0])
                        .footer(|f| {
                            f.text("Powered by Google Maps").icon_url(
                                "https://cdn.iconscout.com/icon/free/png-256/google-map-461800.png",
                            )
                        })
                        .color(0x4285F4);

                    for row in data.rows.iter() {
                        for (i, element) in row.elements.iter().enumerate() {
                            e.field(
                                DESTINATIONS[i][0],
                                format!("{} ({})", element.distance.text, element.duration.text),
                                true,
                            );
                        }
                    }
                    e
                })
            })
            .await
            .map_err(|e| CommandResponse::ComplexFailure {
                response: String::from("Failed to edit interaction response"),
                kind: FailureMessageKind::Error,
                log_message: format!("Failed to edit interaction response: {}", e),
            })?;

        Ok(CommandResponse::NoResponse) // we are handling the response ourselves
    }
}
