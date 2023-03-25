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

pub struct NukeCommand;

#[async_trait]
impl SlashCommand for NukeCommand {
    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| {
            command
            .name("nuke")
            .description("Command for nuking an entire channel with a timeout nuke.")
            .create_option(|sub_command| {
                sub_command
                    .name("minutes-in-past")
                    .description("Timeframe of messages.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|sub_command| {
                sub_command
                    .name("nuclear-fallout-time")
                    .description("The duration of the nuclear fallout in minutes (timeout).")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
        });
        
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
