use serenity::{builder::CreateApplicationCommand, model::prelude::{interaction::application_command::{self, ApplicationCommandInteraction}, command::CommandOptionType}, prelude::Context};

use super::SlashCommand;

pub struct Timeout;

impl Timeout {
    fn build_set_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("timeout")
            .description("Command group for timeouts.")
            .create_option(|sub_command| {
                sub_command
                    .name("set") 
                    .description("Timeouts a user.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("member")
                            .description("The member which should receive a timeout")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("duration")
                            .description("How long the member should be timeouted.")
                            .kind(CommandOptionType::Integer)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason for the timeout.")
                            .kind(CommandOptionType::String)
                            .required(false)
                    })
            })
    }

    fn build_revoke_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("timeout")
            .description("Command group for timeouts.")
            .create_option(|sub_command| {
                sub_command
                    .name("revoke") 
                    .description("Revokes a timeout of from a user.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("member")
                            .description("The member whose timeout should be revoked.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason for revoking the timeout.")
                            .kind(CommandOptionType::String)
                            .required(false)
                    })
            })
    }
}

impl SlashCommand for Timeout {
    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        Self::build_set_command(command);
        Self::build_revoke_command(command)
    }

    fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        database: &sqlx::PgPool,
    ) -> Result<(), super::CommandError> {
        todo!()
    }
}