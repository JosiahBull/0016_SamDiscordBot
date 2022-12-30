use log::error;
use serenity::{
    all::{CommandInteraction, CommandOptionType, ResolvedValue},
    async_trait,
    builder::{
        CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage, EditMessage,
    },
    prelude::Context,
};

use crate::{
    discord_bot::database::shopping::{SerenityShoppingDatabase, ShoppingListItem},
    state::AppState,
};

use super::{command::Command, util::CommandResponse};

pub struct Shop;

impl<'a> TryFrom<&'a CommandInteraction> for Shop {
    type Error = String;
    fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

#[async_trait]
impl<'a> Command<'a> for Shop {
    fn name() -> &'static str {
        "shop"
    }

    fn description() -> &'static str {
        "add an item to the shopping list"
    }

    fn get_application_command_options(cmd: CreateCommand) -> CreateCommand {
        cmd.add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "item",
                "The item to add to the shopping list",
            )
            .required(true)
            .max_length(200)
            .to_owned(),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "personal",
                "true if the item is just for you",
            )
            .required(true),
        )
        .add_option({
            let mut cmd = CreateCommandOption::new(
                CommandOptionType::Integer,
                "quantity",
                "The quantity of the item to add to the shopping list",
            )
            .required(false);

            for i in 1..26 {
                cmd = cmd.add_int_choice(i.to_string(), i);
            }
            cmd
        })
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "store",
                "If the item is to be bought or found in a particular store",
            )
            .required(false)
            .set_autocomplete(true),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "notes",
            "Notes about the item to add to the shopping list",
        ))
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        state: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        // parse the values from the interaction

        let options = interaction.data.options();

        let mut item: Option<&str> = None;
        let mut personal: Option<bool> = None;
        let mut quantity: Option<i64> = None;
        let mut store: Option<&str> = None;
        let mut notes: Option<&str> = None;

        for option in options.into_iter() {
            match (option.name, option.value) {
                ("item", ResolvedValue::String(val)) => item = Some(val),
                ("personal", ResolvedValue::Boolean(val)) => personal = Some(val),
                ("quantity", ResolvedValue::Integer(val)) => quantity = Some(val),
                ("store", ResolvedValue::String(val)) => store = Some(val),
                ("notes", ResolvedValue::String(val)) => notes = Some(val),
                (opt, val) => {
                    panic!("unexpected option name: `{}` and value `{:?}`", opt, val)
                }
            }
        }

        //check item and personal are present
        if item.is_none() || personal.is_none() {
            return Err(CommandResponse::BasicFailure(
                "item and personal are required".to_string(),
            ));
        }
        let item = item.unwrap();
        let personal = personal.unwrap();

        if item.is_empty() {
            return Err(CommandResponse::BasicFailure(
                "item cannot be empty".to_string(),
            ));
        }

        if let Err(e) = interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::Defer(
                    CreateInteractionResponseMessage::new()
                        .content("communicating with database, please wait..."),
                ),
            )
            .await
        {
            return Err(CommandResponse::InternalFailure(format!(
                "error communicating with database: {}",
                e
            )));
        }

        let mut loading_message = match interaction.get_response(&ctx).await {
            Ok(m) => m,
            Err(e) => {
                return Err(CommandResponse::InternalFailure(format!(
                    "error communicating with database: {}",
                    e
                )));
            }
        };

        let user_id: u64 = interaction.user.id.into();
        let loading_message_id: u64 = loading_message.id.into();

        if let Err(e) = state
            .add_shopping_list_item(
                user_id,
                loading_message_id,
                ShoppingListItem {
                    item,
                    personal,
                    quantity,
                    store,
                    notes,
                },
            )
            .await
        {
            error!("error adding shopping list item: {}", e);
            if let Err(inner_e) = loading_message
                .edit(&ctx, EditMessage::new().content("internal error, see logs"))
                .await
            {
                error!("error editing message to return error: {}", inner_e);
            }
            return Err(CommandResponse::NoResponse);
        }

        if let Err(e) = loading_message
            .edit(
                &ctx,
                EditMessage::new().content("").embed(CreateEmbed::new()),
            )
            .await
        {
            error!("error editing message to return success: {}", e);
        }

        Ok(CommandResponse::NoResponse)
    }
}
