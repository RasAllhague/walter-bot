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

use super::SlashCommand;

pub struct TimeoutCommand;

impl TimeoutCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
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

#[async_trait]
impl SlashCommand for TimeoutCommand {
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
    ) -> Result<(), super::CommandError> {
        warn!("Not implemented!");

        Ok(())
    }
}
