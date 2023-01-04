use std::collections::HashSet;

use serenity::{
    all::{
        AutocompleteOption, ButtonStyle, CommandInteraction, CommandOptionType,
        ComponentInteraction, ResolvedValue,
    },
    async_trait,
    builder::{
        AutocompleteChoice, CreateActionRow, CreateAttachment, CreateAutocompleteResponse,
        CreateButton, CreateCommand, CreateCommandOption, CreateEmbed, CreateEmbedFooter,
        CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage,
    },
    json::Value,
    model::prelude::Attachment,
    prelude::Context,
};

use crate::{
    discord_bot::common::embed::EmbedColor,
    state::{AppState, Flatmate, FLATMATES, HEAD_TENANT_ACC_NUMBER, PHRASES},
};

use super::{
    command::{AutocompleteCommand, Command, InteractionCommand},
    util::CommandResponse,
};

async fn handle_autocomplete_for_pay<'c>(
    interaction: &'c CommandInteraction,
    autocomplete: &'c AutocompleteOption<'_>,
) -> Result<CreateAutocompleteResponse, CommandResponse> {
    let mut response = CreateAutocompleteResponse::new();

    // match over which option is focussed, and provide options for that
    match autocomplete.name {
        "purpose" => {
            response = response.set_choices(vec![
                AutocompleteChoice {
                    name: String::from("Food"),
                    value: Value::from("food"),
                },
                AutocompleteChoice {
                    name: String::from("Power"),
                    value: Value::from("power"),
                },
                AutocompleteChoice {
                    name: String::from("Water"),
                    value: Value::from("water"),
                },
                AutocompleteChoice {
                    name: String::from("Internet/Wifi"),
                    value: Value::from("internet"),
                },
            ]);
        }
        i if FLATMATES.iter().any(|f| f.name.to_ascii_lowercase() == *i) => {
            // load all previous values that have been entered as options, and use those as the options for payment
            // this will allow for easy re-use of values

            let mut existing_options: HashSet<String> = HashSet::default();

            for option in interaction.data.options().iter_mut() {
                if matches!(option.value, ResolvedValue::Unresolved(_)) {
                    continue;
                }

                match option.name {
                    "purpose" | "receipt" => {
                        continue;
                    }
                    i => {
                        if let ResolvedValue::Number(num) = option.value {
                            let num_str = format!("{:.2}", num);
                            // check if existing num is in the list of options
                            if !existing_options.contains(&num_str) {
                                response = response
                                    .add_number_choice(format!("***REMOVED***e as {} ({:.2})", i, num), num);
                                existing_options.insert(num_str);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            return Err(CommandResponse::InternalFailure(
                "Invalid autocomplete option".to_string(),
            ));
        }
    }

    Ok(response)
}

async fn create_response<'a>(
    purpose: &str,
    user: &str,
    receipt: &str,
    total: f64,
    amounts: Vec<(&Flatmate<'a>, f64)>,
    account: &str,
    ctx: &Context,
) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(
                CreateEmbed::new()
                    .title("Bill created")
                    .description(format!(
                        "Bill for {} totalling ${:.2} created by {} on {} to be paid into `{}`",
                        purpose,
                        total,
                        user,
                        chrono::offset::Local::now().format("%d/%m/%y at %I:%M%P"),
                        account
                    ))
                    .color(EmbedColor::Red as u32)
                    .fields({
                        let mut fields: Vec<(String, String, bool)> =
                            Vec::with_capacity(amounts.len());
                        for (flatmate, amount) in amounts {
                            if amount == 0.0 {
                                continue;
                            }

                            fields.push((
                                format!("Amount for {} to pay:", flatmate.display_name),
                                format!("${:.2}", amount),
                                false,
                            ));
                        }

                        fields
                    })
                    .footer(CreateEmbedFooter::new(format!(
                        "\n{}",
                        PHRASES[rand::random::<usize>() % PHRASES.len()]
                    ))),
            )
            .add_file(CreateAttachment::url(ctx, receipt).await.unwrap()) //XXX: handle error
            .components({
                let mut components = Vec::with_capacity(2);
                components.push(CreateActionRow::Buttons({
                    vec![
                        CreateButton::new("paid")
                            .style(ButtonStyle::Success)
                            .label("Paid!"),
                        CreateButton::new_link(receipt).label("Receipt"),
                    ]
                }));
                components
            }),
    )
}

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
                .required(true)
                .set_autocomplete(true),
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
                .required(true)
                .set_autocomplete(true),
            );
        }

        cmd.add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "account",
                "The account number to pay into, defaults to head tenant account.",
            )
            .required(false),
        )
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        _: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        // extract the options
        let options = interaction.data.options();

        let mut purpose: Option<&str> = None;
        let mut receipt: Option<&Attachment> = None;
        let mut amount = 0.0;
        let mut amounts: Vec<(&Flatmate, f64)> = Vec::with_capacity(FLATMATES.len());
        let mut account = HEAD_TENANT_ACC_NUMBER;

        for option in options.iter() {
            match option.name {
                "purpose" => {
                    if let ResolvedValue::String(s) = option.value {
                        purpose = Some(s);
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse purpose as a string".to_string(),
                        ));
                    }
                }
                "receipt" => {
                    if let ResolvedValue::Attachment(attachment) = option.value {
                        receipt = Some(attachment);
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse receipt as an attachment".to_string(),
                        ));
                    }
                }
                "account" => {
                    if let ResolvedValue::String(s) = option.value {
                        account = s;
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse account as a string".to_string(),
                        ));
                    }
                }
                _ => {
                    let name = option.name;

                    if let ResolvedValue::Number(value) = option.value {
                        amount += value;
                        amounts.push((FLATMATES.iter().find(|f| f.name == name).unwrap(), value));
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse amount as a number".to_string(),
                        ));
                    }
                }
            }
        }

        // check if initialisation was successful
        if purpose.is_none() || receipt.is_none() || amounts.is_empty() {
            return Err(CommandResponse::InternalFailure(
                "Failed to initialize command".to_string(),
            ));
        }

        let purpose = purpose.unwrap();
        let receipt = receipt.unwrap();

        if let Err(e) = interaction
            .create_response(
                &ctx,
                create_response(
                    purpose,
                    &interaction.user.name,
                    &receipt.url,
                    amount,
                    amounts,
                    account,
                    ctx,
                )
                .await,
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
    async fn answerable<'b>(
        interaction: &'b ComponentInteraction,
        _: &'b AppState,
        _: &'b Context,
    ) -> bool {
        //XXX: eventually it'll be good to store message id's in the database, and react to those *specifically*
        if let Some(embed) = interaction.message.embeds.get(0) {
            if let Some(description) = embed.description.as_ref() {
                return description.starts_with("Bill for ");
            }
        }
        false
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
        let user: Option<&Flatmate> = FLATMATES
            .iter()
            .find(|flatmate| flatmate.discord_id == user);

        if user.is_none() {
            return Err(CommandResponse::InternalFailure(
                "Failed to get user".to_string(),
            ));
        }
        let user = user.unwrap();
        let mut message = interaction.message.clone();
        let current_time = chrono::offset::Local::now().format("%d/%m/%y at %I:%M%P");
        let mut all_set = 0;

        if message.embeds.len() != 1 {
            return Err(CommandResponse::InternalFailure(
                "Invalid embeds in message".to_string(),
            ));
        }

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
                        .footer(CreateEmbedFooter::new({
                            interaction.message.embeds[0]
                                .footer
                                .as_ref()
                                .expect("footer to be present")
                                .text
                                .clone()
                        }))
                        .fields({
                            let mut fields: Vec<(String, String, bool)> =
                                Vec::with_capacity(message.embeds[0].fields.len());
                            for field in message.embeds[0].fields.iter() {
                                if field.name.contains("paid") {
                                    all_set += 1;
                                }
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
                                    all_set += 1;
                                } else {
                                    fields.push((
                                        field.name.clone(),
                                        field.value.clone(),
                                        field.inline,
                                    ));
                                }
                            }
                            fields
                        })
                        .color({
                            if all_set == message.embeds[0].fields.len() {
                                EmbedColor::Green as u32
                            } else {
                                EmbedColor::Red as u32
                            }
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
                        .content(format!(
                            "{}{} paid!",
                            user.name[0..1].to_uppercase(),
                            &user.name[1..]
                        ))
                        .ephemeral(true),
                ),
            )
            .await
            .unwrap();

        Ok(CommandResponse::NoResponse)
    }
}

#[async_trait]
impl<'a> AutocompleteCommand<'a> for PayCommand {
    async fn autocomplete<'c>(
        interaction: &'c CommandInteraction,
        autocomplete: &'c AutocompleteOption,
        _: &'c AppState,
        _: &'c Context,
    ) -> Result<CreateAutocompleteResponse, CommandResponse> {
        handle_autocomplete_for_pay(interaction, autocomplete).await
    }
}

/// A variation of the PayCommand which takes a single argument, and pays that amount to all flatmates
pub struct PayAllCommand {}

impl<'a> TryFrom<&'a CommandInteraction> for PayAllCommand {
    type Error = String;

    fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

#[async_trait]
impl<'a> Command<'a> for PayAllCommand {
    fn name() -> &'static str {
        "pay-all"
    }

    fn description() -> &'static str {
        "Evenly split a bill between all flatmates"
    }

    fn get_application_command_options(cmd: CreateCommand) -> CreateCommand {
        cmd.add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "purpose",
                "What the payment is for",
            )
            .required(true)
            .set_autocomplete(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Attachment,
                "receipt",
                "A receipt for the payment",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::Number, "amount", "The amount to pay")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "account",
                "The account number to pay into, defaults to head tenant account.",
            )
            .required(false),
        )
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        _: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        // extract the options
        let options = interaction.data.options();

        let mut purpose: Option<&str> = None;
        let mut receipt: Option<&Attachment> = None;
        let mut amount: Option<f64> = None;
        let mut account: &str = HEAD_TENANT_ACC_NUMBER;

        for option in options.iter() {
            match option.name {
                "purpose" => {
                    if let ResolvedValue::String(s) = option.value {
                        purpose = Some(s);
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse purpose as a string".to_string(),
                        ));
                    }
                }
                "receipt" => {
                    if let ResolvedValue::Attachment(attachment) = option.value {
                        receipt = Some(attachment);
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse receipt as an attachment".to_string(),
                        ));
                    }
                }
                "amount" => {
                    if let ResolvedValue::Number(n) = option.value {
                        amount = Some(n);
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse amount as a number".to_string(),
                        ));
                    }
                }
                "account" => {
                    if let ResolvedValue::String(s) = option.value {
                        account = s;
                    } else {
                        return Err(CommandResponse::InternalFailure(
                            "Failed to parse account as a string".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(CommandResponse::InternalFailure(
                        "Invalid option".to_string(),
                    ));
                }
            }
        }

        // check all values found
        if purpose.is_none() || amount.is_none() || receipt.is_none() {
            return Err(CommandResponse::InternalFailure(
                "No purpose provided".to_string(),
            ));
        }
        let purpose = purpose.unwrap();
        let amount = amount.unwrap();
        let receipt = receipt.unwrap();

        // parse response and create message
        let mut amounts: Vec<(&Flatmate<'_>, f64)> = Vec::with_capacity(FLATMATES.len());
        let individual = amount / FLATMATES.len() as f64;
        for flatmate in FLATMATES {
            amounts.push((flatmate, individual));
        }

        if let Err(e) = interaction
            .create_response(
                &ctx,
                create_response(
                    purpose,
                    &interaction.user.name,
                    &receipt.url,
                    amount,
                    amounts,
                    account,
                    ctx,
                )
                .await,
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
impl<'a> AutocompleteCommand<'a> for PayAllCommand {
    async fn autocomplete<'c>(
        interaction: &'c CommandInteraction,
        autocomplete: &'c AutocompleteOption,
        _: &'c AppState,
        _: &'c Context,
    ) -> Result<CreateAutocompleteResponse, CommandResponse> {
        handle_autocomplete_for_pay(interaction, autocomplete).await
    }
}
