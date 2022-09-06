use serenity::{model::{Permissions, prelude::{interaction::{application_command::ApplicationCommandInteraction, autocomplete::AutocompleteInteraction}, command::CommandType}}, async_trait, builder::{CreateApplicationCommand, CreateAutocompleteResponse, CreateApplicationCommands}, prelude::Context};

use crate::{AppState, discord_bot::commands::{hide::HideCommand, ping::PingCommand}};

use super::util::CommandResponse;

const DEFAULT_PERMISSIONS: Permissions = Permissions::ADMINISTRATOR;

/// A command that can be used in a guild, restricted to administrators
#[async_trait]
pub trait Command<'a>: TryFrom<&'a ApplicationCommandInteraction> {
    /// Get the name of the command
    fn name() -> &'static str;

    /// Get the description of the command
    fn description() -> &'static str;

    /// Get the discord defined usage of this command, to be sent to discord
    fn get_application_command_options(command: &mut CreateApplicationCommand);

    /// handle the execution of this application command
    async fn handle_application_command<'b> (
        self,
        interaction: &'b ApplicationCommandInteraction,
        app_state: &'b AppState,
        context: &'b Context,
    ) -> Result<CommandResponse<'b>, CommandResponse<'b>>;
}

/// A command that has support for autocomplete responses
#[async_trait]
pub trait AutocompleteCommand<'a>: Command<'a> {
    /// get the autocomplete options for this command, given the current input
    async fn autocomplete<'c>(
        message: &'c AutocompleteInteraction,
        app_state: &'c AppState,
        context: &'c Context,
    ) -> Result<CreateAutocompleteResponse, CommandResponse<'c>>;
}

/// match against a list of provided command types, and generate an application command that can be registered with discord
macro_rules! application_command {
    ( $base:expr, $( $x:ty ),* $(,)? ) => {
        {
            /// ensures that the provided type has relevant traits
            fn assert_command<'a, T: Command<'a, Error=String>>() {}
            $(
                assert_command::<$x>();
                let v_base: &mut CreateApplicationCommands = $base;
                v_base.create_application_command(|command| {
                    <$x>::get_application_command_options(command);
                    command
                        .name(<$x>::name())
                        .description(<$x>::description())
                        .default_member_permissions(DEFAULT_PERMISSIONS)
                        .dm_permission(false)
                        .kind(CommandType::ChatInput)
                });
            )*
        }
    };
}

/// match against a list of provided command types, and produce a response which can be sent to the user
macro_rules! command {
    ( $cmd:expr, $state:expr, $context:expr, $( $x:ty ),* $(,)? ) => {
        {
            /// ensures that the provided type has relevant traits
            fn assert_command<'a, T: Command<'a, Error=String>>() {}
            $(
                assert_command::<$x>();
                if ($cmd).data.name == <$x>::name() {
                    if let Ok(value) = <$x>::try_from($cmd) {
                        return value.handle_application_command($cmd, $state, $context).await
                    }
                }
            )*
            Err(CommandResponse::InternalFailure(String::from("Unsupported Command")))
        }
    };
}

/// match against a list of provided autocomplete command types, and produce a response which can be sent to the user
macro_rules! autocomplete {
    ( $cmd:expr, $state:expr, $context:expr, $( $x:ty ),* $(,)? ) => {
        {
            /// ensures that the provided type has relevant traits
            fn assert_autocomplete<'a, T: AutocompleteCommand<'a, Error=String>>() {}
            $(
                assert_autocomplete::<$x>();
                if ($cmd).data.name == <$x>::name() {
                    return <$x>::autocomplete($cmd, $state, $context).await
                }
            )*
            Err(CommandResponse::InternalFailure(String::from("Unsupported Command")))
        }
    };
}

pub fn application_command() -> CreateApplicationCommands {
    let mut base = CreateApplicationCommands::default();
    application_command!(&mut base, HideCommand, PingCommand);
    base
}

pub async fn command<'a> (
    command: &'a ApplicationCommandInteraction,
    app_state: &'a AppState,
    context: &'a Context,
) -> Result<CommandResponse<'a>, CommandResponse<'a>> {
    command!(command, app_state, context, HideCommand, PingCommand)
}

#[allow(dead_code, unused_variables)]
pub async fn autocomplete<'a> (
    command: &'a AutocompleteInteraction,
    app_state: &'a AppState,
    context: &'a Context,
) -> Result<CreateAutocompleteResponse, CommandResponse<'a>> {
    autocomplete!(command, app_state, context, )
}