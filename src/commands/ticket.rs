use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

use super::SlashCommand;

pub struct TicketCommand;

impl SlashCommand for TicketCommand {
    fn register<'a>(&'a self, command: &'a mut serenity::builder::CreateApplicationCommand) -> &mut serenity::builder::CreateApplicationCommand {
        Self::build_close_command(command);
        Self::build_create_command(command);
        Self::build_invite_command(command)
    }

    fn dispatch(
        &self,
        command: &serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction,
        ctx: &serenity::prelude::Context,
        database: &sqlx::PgPool,
    ) -> Result<(), super::CommandError> {
        todo!()
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
                    .name("create")
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