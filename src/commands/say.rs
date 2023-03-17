use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::ApplicationCommandInteraction,
    },
    prelude::Context,
};

use super::{CommandError, SlashCommand};

pub struct SayCommand;

impl SlashCommand for SayCommand {
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
            .name("say")
            .description("Command for sending a message as the bot.")
            .create_option(|sub_command| {
                sub_command
                    .name("message")
                    .description("The message which should be send.")
                    .kind(CommandOptionType::String)
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
