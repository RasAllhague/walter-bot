use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};

use super::{CommandError, SlashCommand};

pub struct NukeCommand;

impl SlashCommand for NukeCommand {
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
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
    }

    fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        database: &sqlx::PgPool,
    ) -> Result<(), CommandError> {
        todo!()
    }
}
