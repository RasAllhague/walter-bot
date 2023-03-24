use serenity::{builder::CreateApplicationCommand, model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};

use super::{SlashCommand, CommandError};

pub struct LssCommand;

impl SlashCommand for LssCommand {
    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        Self::build_lss_command(command)
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

impl LssCommand {
    fn build_lss_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("lls")
            .description("Command for sending lls images.")
    }
}