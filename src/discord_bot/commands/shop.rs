use std::{cmp::Ordering, collections::HashSet};

use log::error;
use serenity::{
    all::{
        AutocompleteOption, ChannelId, CommandInteraction, CommandOptionType, ComponentInteraction,
        GuildId, Message, ResolvedValue,
    },
    async_trait,
    builder::{
        AutocompleteChoice, CreateActionRow, CreateAutocompleteResponse, CreateButton,
        CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateMessage,
        EditMessage,
    },
    prelude::Context,
};

use crate::{
    discord_bot::{
        common::embed::EmbedColor,
        database::shopping::{NewShoppingListItem, SerenityShoppingDatabase},
    },
    state::AppState,
};

use super::{
    command::{AutocompleteCommand, Command, InteractionCommand},
    util::CommandResponse,
};

const EXTRA_STORE_NAMES: &[&str] = &[
    "Pack'n'Save",
    "Countdown",
    "Bunnings",
    "Mitre 10",
    "The Warehouse",
    "Kmart",
    "Farmers",
];

const EXTRA_ITEMS: &[&str] = &[
    "milk 2L",
    "loaf of bread",
    "12 eggs",
    "cheese 1kg",
    "butter",
    "chocolate",
    "coffee",
    "tea",
    "sugar",
    "flour",
    "oil",
    "x2 can of tomatoes",
    "fresh tomatoes",
    "cherry tomatoes",
    "brown onions",
    "red onions",
    "potatoes",
    "carrots",
    "general fruit and vege",
    "chicken breast 500g",
    "beef mince 500g",
    "pork mine 500g",
    "white fish",
    "hoki crumbed fish",
    "orange juice (pulp)",
    "orange juice (no pulp)",
    "toilet paper",
    "paper towels",
    "dishwashing liquid",
    "dishwasher powder",
    "washing powder",
    "napisan powder",
    "bleach",
    "toothpaste",
    "toothbrush",
    "shampoo",
    "conditioner",
    "soap",
    "deodorant",
    "razors",
    "shaving cream",
    "hair gel",
    "band-aids",
    "painkillers",
    "antibiotics",
    "vitamins",
    "protein powder",
    "banana",
    "apple",
    "orange",
    "kiwi fruit",
    "lemon",
    "lime",
    "avocado",
    "cucumber",
    "lettuce",
    "capsicum",
    "zucchini",
    "broccoli",
    "cauliflower",
    "asparagus",
    "corn",
    "mushrooms",
    "spinach",
    "tomato",
];

#[async_trait]
trait Interactable: Sync {
    async fn interactable_create_response(
        &self,
        ctx: &Context,
        response: CreateInteractionResponse,
    ) -> Result<(), serenity::Error>;

    async fn interactable_create_followup(
        &self,
        ctx: &Context,
        response: CreateInteractionResponseFollowup,
    ) -> Result<Message, serenity::Error>;

    async fn interactable_get_response(&self, ctx: &Context) -> Result<Message, serenity::Error>;

    fn user(&self) -> &serenity::model::user::User;
    fn channel_id(&self) -> ChannelId;
    fn guild_id(&self) -> Option<GuildId>;
}

#[async_trait]
impl Interactable for CommandInteraction {
    async fn interactable_create_response(
        &self,
        ctx: &Context,
        response: CreateInteractionResponse,
    ) -> Result<(), serenity::Error> {
        self.create_response(ctx, response).await
    }

    async fn interactable_create_followup(
        &self,
        ctx: &Context,
        response: CreateInteractionResponseFollowup,
    ) -> Result<Message, serenity::Error> {
        self.create_followup(ctx, response).await
    }

    async fn interactable_get_response(&self, ctx: &Context) -> Result<Message, serenity::Error> {
        self.get_response(ctx).await
    }

    fn user(&self) -> &serenity::model::user::User {
        &self.user
    }

    fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }
}

#[async_trait]
impl Interactable for ComponentInteraction {
    async fn interactable_create_response(
        &self,
        ctx: &Context,
        response: CreateInteractionResponse,
    ) -> Result<(), serenity::Error> {
        self.create_response(ctx, response).await
    }

    async fn interactable_create_followup(
        &self,
        ctx: &Context,
        response: CreateInteractionResponseFollowup,
    ) -> Result<Message, serenity::Error> {
        self.create_followup(ctx, response).await
    }

    async fn interactable_get_response(&self, ctx: &Context) -> Result<Message, serenity::Error> {
        self.get_response(ctx).await
    }

    fn user(&self) -> &serenity::model::user::User {
        &self.user
    }

    fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }
}

trait Constructable: Default {
    fn add_embed(self, embed: CreateEmbed) -> Self;
    fn add_components(self, components: Vec<CreateActionRow>) -> Self;
}

impl Constructable for CreateInteractionResponseFollowup {
    fn add_embed(self, embed: CreateEmbed) -> Self {
        self.embed(embed)
    }

    fn add_components(self, components: Vec<CreateActionRow>) -> Self {
        self.components(components)
    }
}

impl Constructable for CreateMessage {
    fn add_embed(self, embed: CreateEmbed) -> Self {
        self.embed(embed)
    }

    fn add_components(self, components: Vec<CreateActionRow>) -> Self {
        self.components(components)
    }
}

async fn create_new_shopping<'b, A: Interactable, B: Constructable>(
    shop: Shop<'b>,
    interaction: &'b A,
    state: &'b AppState,
    ctx: &'b Context,
) -> Result<B, CommandResponse> {
    if let Err(e) = interaction
        .interactable_create_response(
            ctx,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await
    {
        return Err(CommandResponse::InternalFailure(format!(
            "error communicating with database: {}",
            e
        )));
    }

    let loading_message = match interaction.interactable_get_response(ctx).await {
        Ok(m) => m,
        Err(e) => {
            return Err(CommandResponse::InternalFailure(format!(
                "error communicating with database: {}",
                e
            )));
        }
    };

    let user_id: u64 = interaction.user().id.into();
    let loading_message_id: u64 = loading_message.id.into();
    let channel_id: u64 = interaction.channel_id().into();
    let guild_id: Option<u64> = interaction.guild_id().map(|id| id.into());

    if let Err(e) = state
        .add_shopping_list_item(
            user_id,
            loading_message_id,
            channel_id,
            guild_id,
            NewShoppingListItem {
                item: shop.item,
                personal: shop.personal,
                quantity: shop.quantity,
                store: shop.store,
                notes: shop.notes,
            },
        )
        .await
    {
        error!("error adding shopping list item: {}", e);
        if let Err(inner_e) = interaction
            .interactable_create_followup(
                ctx,
                CreateInteractionResponseFollowup::new()
                    .content("error communicating with database")
                    .ephemeral(true),
            )
            .await
        {
            error!("error editing message to return error: {}", inner_e);
        }
        return Err(CommandResponse::NoResponse);
    }

    Ok(B::default()
        .add_embed(
            CreateEmbed::new()
                // .title("Added to shopping list") //XXX: experiment
                .description(format!(
                    "Added x{} {}{} to the shopping list{}{}",
                    shop.quantity,
                    shop.item,
                    if shop.personal { " (personal)" } else { "" },
                    if shop.store.is_some() {
                        format!(" from {}", shop.store.unwrap())
                    } else {
                        "".to_string()
                    },
                    if shop.notes.is_some() {
                        format!("\n**note:** {}", shop.notes.unwrap())
                    } else {
                        "".to_string()
                    },
                ))
                .color(EmbedColor::Red as u32),
        )
        .add_components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("bought")
                .style(serenity::all::ButtonStyle::Success)
                .label("Bought"),
            CreateButton::new("remove")
                .style(serenity::all::ButtonStyle::Danger)
                .label("Remove"),
            CreateButton::new("readd")
                .style(serenity::all::ButtonStyle::Secondary)
                .label("Re-add")
                .disabled(true),
        ])]))
}

pub struct Shop<'a> {
    item: &'a str,
    personal: bool,
    quantity: i64,
    store: Option<&'a str>,
    notes: Option<&'a str>,
}

impl<'a> TryFrom<&'a CommandInteraction> for Shop<'a> {
    type Error = String;
    fn try_from(interaction: &'a CommandInteraction) -> Result<Self, Self::Error> {
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

        if item.is_none() || personal.is_none() {
            return Err(String::from("item and personal are required"));
        }
        let item = item.unwrap();
        let personal = personal.unwrap();
        let quantity = quantity.unwrap_or(1);

        Ok(Shop {
            item,
            personal,
            quantity,
            store,
            notes,
        })
    }
}

#[async_trait]
impl<'a> Command<'a> for Shop<'a> {
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
            .set_autocomplete(true)
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
            .set_autocomplete(true)
            .max_length(100)
            .to_owned(),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "notes",
                "Notes about the item to add to the shopping list",
            )
            .required(false)
            .max_length(100)
            .to_owned(),
        )
    }

    async fn handle_application_command<'b>(
        self,
        interaction: &'b CommandInteraction,
        state: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        let resp = create_new_shopping(self, interaction, state, ctx).await?;

        if let Err(e) = interaction.create_followup(&ctx, resp).await {
            error!("error creating followup: {}", e);
            return Err(CommandResponse::NoResponse);
        }
        Ok(CommandResponse::NoResponse)
    }
}

#[async_trait]
impl<'a> AutocompleteCommand<'a> for Shop<'a> {
    async fn autocomplete<'c>(
        command: &'c CommandInteraction,
        autocomplete: &'c AutocompleteOption,
        app_state: &'c AppState,
        _: &'c Context,
    ) -> Result<CreateAutocompleteResponse, CommandResponse> {
        let mut response = CreateAutocompleteResponse::new();
        let user_id: u64 = command.user.id.into();

        let mut items = match app_state
            .get_recent_shopping_list_items_by_user(user_id, 50)
            .await
        {
            Ok(items) => items,
            Err(e) => {
                return Err(CommandResponse::InternalFailure(format!(
                    "error communicating with database: {}",
                    e
                )));
            }
        };

        let extra_items = match app_state.get_recent_shopping_list_items(50).await {
            Ok(items) => items,
            Err(e) => {
                return Err(CommandResponse::InternalFailure(format!(
                    "error communicating with database: {}",
                    e
                )));
            }
        };

        for item in extra_items {
            if !items.contains(&item) {
                items.push(item);
            }
        }

        let search_phrase = autocomplete.value;

        match autocomplete.name {
            "item" => {
                let mut item_names: HashSet<String> =
                    items.into_iter().map(|item| item.item).collect();
                item_names.extend(EXTRA_ITEMS.iter().map(|item| item.to_string()));

                //sort item names, preferring items that start with, then contain, the current search phrase
                let mut item_names: Vec<String> = item_names.into_iter().collect();
                item_names.sort_by(|a, b| {
                    let a_start = a.starts_with(search_phrase);
                    let b_start = b.starts_with(search_phrase);
                    let a_contains = a.contains(search_phrase);
                    let b_contains = b.contains(search_phrase);

                    if a_start && !b_start {
                        Ordering::Less
                    } else if !a_start && b_start {
                        Ordering::Greater
                    } else if a_contains && !b_contains {
                        Ordering::Less
                    } else if !a_contains && b_contains {
                        Ordering::Greater
                    } else {
                        a.cmp(b)
                    }
                });
                item_names.truncate(25);

                let choices: Vec<AutocompleteChoice> = item_names
                    .into_iter()
                    .map(|item| AutocompleteChoice {
                        name: item.clone(),
                        value: serde_json::Value::String(item),
                    })
                    .collect();

                response = response.set_choices(choices);
            }
            "store" => {
                let mut store_names: HashSet<String> =
                    items.into_iter().filter_map(|item| item.store).collect();
                store_names.extend(EXTRA_STORE_NAMES.iter().map(|store| store.to_string()));

                //sort store names, preferring stores that start with, then contain, the current search phrase
                let mut store_names: Vec<String> = store_names.into_iter().collect();
                store_names.sort_by(|a, b| {
                    let a_start = a.starts_with(search_phrase);
                    let b_start = b.starts_with(search_phrase);
                    let a_contains = a.contains(search_phrase);
                    let b_contains = b.contains(search_phrase);

                    if a_start && !b_start {
                        Ordering::Less
                    } else if !a_start && b_start {
                        Ordering::Greater
                    } else if a_contains && !b_contains {
                        Ordering::Less
                    } else if !a_contains && b_contains {
                        Ordering::Greater
                    } else {
                        a.cmp(b)
                    }
                });
                store_names.truncate(25);

                let choices: Vec<AutocompleteChoice> = store_names
                    .into_iter()
                    .map(|store| AutocompleteChoice {
                        name: store.clone(),
                        value: serde_json::Value::String(store),
                    })
                    .collect();

                response = response.set_choices(choices);
            }
            _ => {
                return Err(CommandResponse::InternalFailure(
                    "Invalid autocomplete option".to_string(),
                ));
            }
        }

        Ok(response)
    }
}

#[async_trait]
impl<'a> InteractionCommand<'a> for Shop<'a> {
    async fn answerable<'b>(
        interaction: &'b ComponentInteraction,
        app_state: &'b AppState,
        _: &'b Context,
    ) -> bool {
        let msg_id: u64 = interaction.message.id.into();
        match app_state.get_shopping_list_item_by_message_id(msg_id).await {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(e) => {
                error!("error communicating with database: {}", e);
                false
            }
        }
    }

    async fn interaction<'b>(
        interaction: &'b ComponentInteraction,
        app_state: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        let msg_id: u64 = interaction.message.id.into();
        let user_id: u64 = interaction.user.id.into();

        match interaction.data.custom_id.as_ref() {
            "bought" => {
                if let Err(e) = app_state
                    .set_shopping_list_item_bought(user_id, msg_id, true)
                    .await
                {
                    return Err(CommandResponse::InternalFailure(format!(
                        "error communicating with database: {}",
                        e
                    )));
                }

                let ex_embed = match interaction.message.embeds.get(0) {
                    Some(embed) => embed,
                    None => {
                        return Err(CommandResponse::InternalFailure(
                            "error communicating with discord".to_string(),
                        ));
                    }
                };

                let mut edit_message = interaction.message.clone();

                if let Err(e) = edit_message
                    .edit(
                        &ctx,
                        EditMessage::new()
                            .embed(
                                CreateEmbed::new()
                                    //XXX: title?
                                    .description(format!(
                                        "(BOUGHT) ~~{}~~",
                                        ex_embed
                                            .description
                                            .as_ref()
                                            .expect("description not found")
                                    ))
                                    .color(EmbedColor::Green as u32),
                            )
                            .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
                                "readd",
                            )
                            .style(serenity::all::ButtonStyle::Secondary)
                            .label("Re-add")
                            .disabled(false)])]),
                    )
                    .await
                {
                    return Err(CommandResponse::InternalFailure(format!(
                        "error communicating with discord: {}",
                        e
                    )));
                }

                interaction
                    .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                    .await
                    .unwrap();
            }
            "remove" => {
                let ex_embed = match interaction.message.embeds.get(0) {
                    Some(embed) => embed,
                    None => {
                        return Err(CommandResponse::InternalFailure(
                            "error communicating with discord".to_string(),
                        ));
                    }
                };

                let mut edit_message = interaction.message.clone();

                if let Err(e) = edit_message
                    .edit(
                        &ctx,
                        EditMessage::new()
                            .embed(
                                CreateEmbed::new()
                                    .color(EmbedColor::Orange as u32)
                                    .description(format!(
                                        "(REMOVED) {}",
                                        ex_embed
                                            .description
                                            .as_ref()
                                            .expect("description not found")
                                    )),
                            )
                            .components(vec![CreateActionRow::Buttons(vec![CreateButton::new(
                                "readd",
                            )
                            .style(serenity::all::ButtonStyle::Secondary)
                            .label("Re-add")
                            .disabled(false)])]),
                    )
                    .await
                {
                    return Err(CommandResponse::InternalFailure(format!(
                        "error communicating with discord: {}",
                        e
                    )));
                }

                interaction
                    .create_response(&ctx, CreateInteractionResponse::Acknowledge)
                    .await
                    .unwrap();
            }
            "readd" => {
                let item = match app_state.get_shopping_list_item_by_message_id(msg_id).await {
                    Ok(Some(item)) => item,
                    Ok(None) => {
                        return Err(CommandResponse::InternalFailure(
                            "error communicating with database".to_string(),
                        ));
                    }
                    Err(e) => {
                        return Err(CommandResponse::InternalFailure(format!(
                            "error communicating with database: {}",
                            e
                        )));
                    }
                };

                let resp = create_new_shopping(
                    Shop {
                        item: item.item.as_ref(),
                        personal: item.personal,
                        quantity: item.quantity,
                        store: item.store.as_deref(),
                        notes: item.notes.as_deref(),
                    },
                    interaction,
                    app_state,
                    ctx,
                )
                .await?;

                if let Err(e) = interaction.create_followup(&ctx, resp).await {
                    return Err(CommandResponse::InternalFailure(format!(
                        "error communicating with discord: {}",
                        e
                    )));
                }
            }
            _ => {
                return Err(CommandResponse::InternalFailure(
                    "Invalid interaction".to_string(),
                ));
            }
        }

        Ok(CommandResponse::NoResponse)
    }
}

pub struct ShoppingComplete;

impl<'a> TryFrom<&'a CommandInteraction> for ShoppingComplete {
    type Error = String;

    fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
        Ok(ShoppingComplete)
    }
}

#[async_trait]
impl<'a> Command<'a> for ShoppingComplete {
    fn name() -> &'static str {
        "shopping-complete"
    }

    fn description() -> &'static str {
        "Run this command once you have completed shopping"
    }

    fn get_application_command_options(command: CreateCommand) -> CreateCommand {
        command
    }

    async fn handle_application_command<'b>(
        self,
        cmd_interaction: &'b CommandInteraction,
        app_state: &'b AppState,
        ctx: &'b Context,
    ) -> Result<CommandResponse, CommandResponse> {
        if let Err(e) = cmd_interaction.create_response(&ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("-----------------------------------\n**Shopping Complete!**\n-----------------------------------")
            )
        ).await {
            return Err(CommandResponse::InternalFailure(format!(
                "error communicating with discord: {}",
                e
            )));
        }

        // collect every non-bought item from the shopping list
        let items = match app_state.get_unbought_shopping_list_items().await {
            Ok(items) => items,
            Err(e) => {
                return Err(CommandResponse::InternalFailure(format!(
                    "error communicating with database: {}",
                    e
                )));
            }
        };

        let channel = cmd_interaction.channel_id();

        // for each item, send a message to the shopping channel
        for item in items {
            let resp = create_new_shopping(
                Shop {
                    item: item.item.as_ref(),
                    personal: item.personal,
                    quantity: item.quantity,
                    store: item.store.as_deref(),
                    notes: item.notes.as_deref(),
                },
                cmd_interaction,
                app_state,
                ctx,
            )
            .await?;

            if let Err(e) = channel.send_message(&ctx, resp).await {
                return Err(CommandResponse::InternalFailure(format!(
                    "error communicating with discord: {}",
                    e
                )));
            }
        }

        Ok(CommandResponse::NoResponse)
    }
}

// pub struct ShoppingList;

// impl<'a> TryFrom<&'a CommandInteraction> for ShoppingList {
//     type Error = String;
//     fn try_from(_: &'a CommandInteraction) -> Result<Self, Self::Error> {
//         Ok(Self)
//     }
// }

// #[async_trait]
// impl<'a> Command<'a> for ShoppingList {
//     fn name() -> &'static str {
//         "list-all"
//     }

//     fn description() -> &'static str {
//         "List all shopping list items"
//     }

//     fn get_application_command_options(command: CreateCommand) -> CreateCommand {
//         command
//     }

//     async fn handle_application_command<'b>(
//         self,
//         _: &'b CommandInteraction,
//         app_state: &'b AppState,
//         ctx: &'b Context,
//     ) -> Result<CommandResponse, CommandResponse> {
//         let shopping_list = match app_state.get_shopping_list().await {
//             Ok(Some(s)) => s,
//             Ok(None) => {
//                 return Ok(CommandResponse::ComplexSuccess(
//                     CreateInteractionResponse::Message(
//                         CreateInteractionResponseMessage::new()
//                             .content("No shopping list found")
//                             .ephemeral(true),
//                     ),
//                 ))
//             }
//             Err(e) => {
//                 return Err(CommandResponse::InternalFailure(format!(
//                     "error communicating with database: {}",
//                     e
//                 )));
//             }
//         };
//         let (shopping_list, shopping_items) = shopping_list;

//         let msg = MessageId::from(shopping_list.creation_message_id as u64);
//         let msg_link = msg
//             .link_ensured(
//                 &ctx,
//                 ChannelId::from(shopping_list.creation_message_channel_id as u64),
//                 shopping_list
//                     .creation_message_guild_id
//                     .map(|g| GuildId::from(g as u64)),
//             )
//             .await;

//         Ok(CommandResponse::ComplexSuccess(
//             CreateInteractionResponse::Message(
//                 CreateInteractionResponseMessage::new()
//                     .embed(
//                         CreateEmbed::new()
//                             .description(format!(
//                                 "Shopping list dated `{}`",
//                                 chrono::offset::Local::now().format("%d/%m/%y at %I:%M%P")
//                             ))
//                             .fields({
//                                 let mut fields: Vec<(String, String, bool)> =
//                                     Vec::with_capacity(shopping_items.len());
//                                 for item in shopping_items.into_iter() {
//                                     fields.push((
//                                         item.item,
//                                         format!(
//                                             "{} {}{}{}",
//                                             item.quantity,
//                                             if item.personal { " (personal)" } else { "" },
//                                             item.store
//                                                 .map(|s| format!(" {}", s))
//                                                 .unwrap_or_default(),
//                                             item.notes
//                                                 .map(|n| format!(" {}", n))
//                                                 .unwrap_or_default(),
//                                         ),
//                                         false,
//                                     ))
//                                 }
//                                 fields
//                             })
//                             .footer(CreateEmbedFooter::new(format!(
//                                 "\n{}",
//                                 PHRASES[rand::random::<usize>() % PHRASES.len()]
//                             ))),
//                     )
//                     .components(vec![CreateActionRow::Buttons(vec![
//                         CreateButton::new("prev")
//                             .style(serenity::all::ButtonStyle::Secondary)
//                             .label("Prev")
//                             .disabled(true), //TODO: setup pagination
//                         CreateButton::new("next")
//                             .style(serenity::all::ButtonStyle::Primary)
//                             .label("Next")
//                             .disabled(false), //TODO: setup pagination
//                         CreateButton::new("refresh")
//                             .style(serenity::all::ButtonStyle::Danger)
//                             .label("Refresh"),
//                         CreateButton::new_link(msg_link).label("First Item (for ticking)"),
//                     ])]),
//             ),
//         ))
//     }
// }

// #[async_trait]
// impl<'a> InteractionCommand<'a> for ShoppingList {
//     async fn answerable<'b>(
//         interaction: &'b ComponentInteraction,
//         _: &'b AppState,
//         _: &'b Context,
//     ) -> bool {
//         if !interaction
//             .message
//             .content
//             .starts_with("Shopping list dated")
//         {
//             return false;
//         }

//         let now = Local::now();
//         let message_time_str = interaction
//             .message
//             .content
//             .strip_prefix("Shopping list dated `")
//             .unwrap()
//             .strip_suffix('`')
//             .unwrap();
//         let message_time =
//             match chrono::DateTime::parse_from_str(message_time_str, "%d/%m/%y at %I:%M%P") {
//                 Ok(t) => t,
//                 Err(_) => return false,
//             };
//         let message_time = message_time.with_timezone(&Local);

//         // check message is less than 24 hours old
//         if now - message_time > chrono::Duration::hours(24) {
//             return false;
//         }
//         return true;
//     }

//     async fn interaction<'b>(
//         interaction: &'b ComponentInteraction,
//         app_state: &'b AppState,
//         context: &'b Context,
//     ) -> Result<CommandResponse, CommandResponse> {
//         match interaction.data.custom_id.as_ref() {
//             "prev" => {}
//             "next" => {}
//             "refresh" => {}
//             _ => {
//                 return Err(CommandResponse::InternalFailure(
//                     "Invalid interaction".to_string(),
//                 ));
//             }
//         }

//         Ok(CommandResponse::NoResponse)
//     }
// }
