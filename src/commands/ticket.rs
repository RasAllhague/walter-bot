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

pub struct TicketCommand;

static COMMAND_NAME: &str = "ticket";

#[async_trait]
impl SlashCommand for TicketCommand {
    fn name(&self) -> String {
        String::from(COMMAND_NAME)
    }

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

impl TicketCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(COMMAND_NAME)
            .description("Commands for ticket creation and management.")
            .create_option(|sub_command| {
                sub_command
                    .name("create")
                    .description("Creates a new ticket for the requesting person.")
                    .kind(CommandOptionType::SubCommand)
            })
            .create_option(|sub_command| {
                sub_command
                    .name("invite")
                    .description("Invites more people to a ticket.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("user")
                            .description("User to invite.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("ticket-id")
                            .description("The id of the ticket you want to invite people to.")
                            .kind(CommandOptionType::Integer)
                            .required(true)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("close")
                    .description("Closes the current ticket and removes non ticket managers from accessing it.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("ticket-id")
                            .description("The id of the ticket you want to invite people to.")
                            .kind(CommandOptionType::Integer)
                            .required(true)
                    })
            })
    }
}
