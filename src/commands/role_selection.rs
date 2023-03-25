use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};
use tracing::log::warn;

use crate::handler::Configuration;

use super::{CommandError, SlashCommand};

pub struct RoleSelectionCommand;

impl RoleSelectionCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("role-selection")
            .description("Commands for configuration role selection.")
            .create_option(|sub_command| {
                sub_command
                    .name("create")
                    .description("Command for creating a role selection message.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("description")
                            .description("The description for the message embed.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("add-role")
                    .description("Adds a role to the role selection.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("role")
                            .description("The role to add to the role selection.")
                            .kind(CommandOptionType::Role)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("text")
                            .description("The display text for the role in the selection.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("remove-role")
                    .description("Removes a role to the role selection.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("role")
                            .description("The role to remove from the role selection.")
                            .kind(CommandOptionType::Role)
                            .required(true)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("reload")
                    .description("Reloads the role select embeds.")
                    .kind(CommandOptionType::SubCommand)
            })
    }
}

#[async_trait]
impl SlashCommand for RoleSelectionCommand {
    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| Self::build(command));

        commands
    }

    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        database: &sqlx::PgPool,
        configuration: &Configuration,
    ) -> Result<(), CommandError> {
        warn!("Not implemented!");

        Ok(())
    }
}
