use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::{
        command::{Command, CommandOptionType},
        interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};
use tracing::log::warn;

use crate::handler::Configuration;

use super::{CommandError, SlashCommand};

pub struct InfractionCommand;

impl InfractionCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("infraction")
            .description("Command group for punishing users.")
            .create_option(|sub_command| {
                sub_command
                    .name("warn")
                    .description("Warns one or more user for a specified reason.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason why the warning was given.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user1")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user2")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user3")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user4")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user5")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user6")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user7")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user8")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("ban")
                    .description("Ban one or more user for a specified reason.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason why the ban was given.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user1")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user2")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user3")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user4")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user5")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user6")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user7")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user8")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("kick")
                    .description("Kick one or more user for a specified reason.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason why the kick was given.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user1")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user2")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user3")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user4")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user5")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user6")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user7")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user8")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
            })
    }
}

#[async_trait]
impl SlashCommand for InfractionCommand {
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
