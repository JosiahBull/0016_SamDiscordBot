use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serenity::{
    all::{
        ButtonStyle, CommandDataOptionValue, CommandInteraction, CommandOptionType,
        ComponentInteraction,
    },
    async_trait,
    builder::{
        CreateActionRow, CreateAttachment, CreateButton, CreateCommand, CreateCommandOption,
        CreateEmbed, CreateEmbedFooter, CreateInteractionResponse,
        CreateInteractionResponseMessage, EditMessage,
    },
    model::prelude::Attachment,
    prelude::Context,
};

use crate::state::{AppState, Flatemate, FLATMATES, PHRASES};

use super::{
    command::{Command, InteractionCommand},
    util::CommandResponse,
};

pub struct PayCommand {}

impl<'a> TryFrom<&'a CommandInteraction> for PayCommand {
    type Error = String;

    fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

#[async_trait]
impl<'a> Command<'a> for PayCommand {
    fn name() -> &'static str {
        "pay"
    }

    fn description() -> &'static str {
        "Create a shared bill for the flat"
    }

    fn get_application_command_options(mut cmd: CreateCommand) -> CreateCommand {
        cmd = cmd
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "purpose",
                    "What is this bill for?",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Attachment,
                    "receipt",
                    "Attach a photograph of the receipt",
                )
                .required(true),
            );

        for flatmate in FLATMATES.iter() {
            cmd = cmd.add_option(
                CreateCommandOption::new(
                    CommandOptionType::Number,
                    flatmate.name.to_ascii_lowercase(),
                    format!("The amount for {} to pay.", flatmate.name),
                )
                .required(true),
            );
        }

        cmd
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        _: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        // extract the options
        let options = &interaction.data.options;

        let mut purpose: Option<&str> = None;
        let mut receipt: Option<&Attachment> = None;
        let mut amount: Vec<(&str, f64)> = Vec::with_capacity(FLATMATES.len());

        for option in options.iter() {
            match option.name.as_str() {
                "purpose" => {
                    purpose = Some(option.value.as_str().unwrap());
                }
                "receipt" if matches!(option.value, CommandDataOptionValue::Attachment(_)) => {
                    if let CommandDataOptionValue::Attachment(attachment) = &option.value {
                        receipt = Some(
                            interaction
                                .data
                                .resolved
                                .attachments
                                .get(attachment)
                                .unwrap(),
                        ); //XXX: handle error
                    }
                }
                "receipt" => {
                    return Err(CommandResponse::InternalFailure(
                        "Failed to parse receipt as an attachment".to_string(),
                    ));
                }
                _ => {
                    let name = option.name.as_str();
                    let value = option.value.as_f64().unwrap();
                    amount.push((name, value));
                }
            }
        }

        // if any names aren't present, add them with a value of 0
        for flatmate in FLATMATES.iter() {
            if !amount.iter().any(|(n, _)| n == &flatmate.name) {
                amount.push((flatmate.name, 0.0));
            }
        }

        // check if initialisation was successful
        if purpose.is_none() || receipt.is_none() || amount.is_empty() {
            return Err(CommandResponse::InternalFailure(
                "Failed to initialize command".to_string(),
            ));
        }

        let purpose = purpose.unwrap();
        let receipt = receipt.unwrap();

        if let Err(e) = interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .embed(
                            CreateEmbed::new()
                                .title("Bill created")
                                .description(format!(
                                    "Bill for {} created by {}",
                                    purpose, interaction.user.name
                                ))
                                .color(0xFF0000)
                                .fields({
                                    let mut fields: Vec<(String, String, bool)> =
                                        Vec::with_capacity(amount.len());
                                    for (name, value) in amount.iter() {
                                        fields.push((
                                            format!(
                                                "Amount for {}{} to pay:",
                                                name[0..1].to_uppercase(),
                                                &name[1..]
                                            ),
                                            format!("${:.2}", value),
                                            false,
                                        ));
                                    }
                                    fields
                                })
                                .footer(CreateEmbedFooter::new(
                                    PHRASES[rand::random::<usize>() % PHRASES.len()],
                                )),
                        )
                        .add_file(CreateAttachment::url(&ctx, &receipt.url).await.unwrap()) //XXX: handle error
                        .components({
                            let mut components = Vec::with_capacity(2);
                            components.push(CreateActionRow::Buttons({
                                vec![
                                    CreateButton::new("paid")
                                        .style(ButtonStyle::Success)
                                        .label("Paid!"),
                                    CreateButton::new_link(&receipt.url).label("Receipt"),
                                ]
                            }));
                            components
                        }),
                ),
            )
            .await
        {
            return Err(CommandResponse::InternalFailure(format!(
                "Failed to create interaction response: {}",
                e
            )));
        }

        Ok(CommandResponse::NoResponse)
    }
}

#[async_trait]
impl<'a> InteractionCommand<'a> for PayCommand {
    fn answerable<'b>(_: &'b ComponentInteraction, _: &'b AppState, _: &'b Context) -> bool {
        true //TODO
    }

    async fn interaction<'b>(
        interaction: &'b ComponentInteraction,
        _: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        if interaction.member.is_none() {
            return Err(CommandResponse::InternalFailure(
                "Failed to get member".to_string(),
            ));
        }

        let user: u64 = interaction.user.id.into();
        let user: Option<&Flatemate> = FLATMATES
            .iter()
            .find(|flatmate| flatmate.discord_id == user);

        if user.is_none() {
            return Err(CommandResponse::InternalFailure(
                "Failed to get user".to_string(),
            ));
        }
        let user = user.unwrap();

        let mut message = interaction.message.clone();

        let current_time = SystemTime::now();
        let current_time: DateTime<Utc> = current_time.into();
        let current_time = current_time.to_rfc2822();

        if let Err(e) = message
            .edit(
                &ctx,
                EditMessage::new().embed(
                    CreateEmbed::new()
                        .description(
                            interaction.message.embeds[0]
                                .description
                                .as_ref()
                                .unwrap_or(&String::from("")),
                        )
                        .color(0xFF0000)
                        .footer(CreateEmbedFooter::new(
                            PHRASES[rand::random::<usize>() % PHRASES.len()],
                        ))
                        .fields({
                            let mut fields: Vec<(String, String, bool)> =
                                Vec::with_capacity(message.embeds[0].fields.len());
                            for field in message.embeds[0].fields.iter() {
                                if field.name.to_lowercase().contains(user.name)
                                    && field.name.contains("pay")
                                {
                                    fields.push((
                                        format!(
                                            "{}{} paid {} on:",
                                            user.name[0..1].to_uppercase(),
                                            &user.name[1..],
                                            field.value
                                        ),
                                        current_time.to_string(),
                                        field.inline,
                                    ));
                                } else {
                                    fields.push((
                                        field.name.clone(),
                                        field.value.clone(),
                                        field.inline,
                                    ));
                                }
                            }
                            fields
                        }),
                ),
            )
            .await
        {
            return Err(CommandResponse::InternalFailure(format!(
                "Failed to edit message: {}",
                e
            )));
        }

        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!("{} paid!", user.name))
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        Ok(CommandResponse::NoResponse)
    }
}
