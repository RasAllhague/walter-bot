use async_trait::async_trait;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};
use tracing::log::warn;

use crate::handler::Configuration;

use super::{CommandError, SlashCommand};

pub struct TicketCommand;

#[async_trait]
impl SlashCommand for TicketCommand {
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        Self::build_close_command(command);
        Self::build_create_command(command);
        Self::build_invite_command(command)
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
    fn build_create_command(
        command: &mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
            .name("ticket")
            .description("Commands for ticket creation and management.")
            .create_option(|sub_command| {
                sub_command
                    .name("create1")
                    .description("Creates a new ticket for the requesting person.")
                    .kind(CommandOptionType::SubCommand)
            })
    }

    fn build_invite_command(
        command: &mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
            .name("ticket")
            .description("Commands for ticket creation and management.")
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
    }

    fn build_close_command(
        command: &mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
            .name("ticket")
            .description("Commands for ticket creation and management.")
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
