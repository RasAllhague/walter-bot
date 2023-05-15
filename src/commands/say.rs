use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommands},
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOption},
                InteractionResponseType,
            },
        },
    },
    prelude::Context,
};


use crate::handler::Configuration;

use super::{CommandError, SlashCommand, parser::PositionalOptionParser};

pub struct SayCommand;

static COMMAND_NAME: &str = "say";

#[async_trait]
impl SlashCommand for SayCommand {
    fn name(&self) -> String {
        String::from(COMMAND_NAME)
    }

    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| {
            command
                .name(COMMAND_NAME)
                .description("Command for sending a message as the bot.")
                .create_option(|sub_command| {
                    sub_command
                        .name("channel")
                        .description("The channel to which this message should be send to.")
                        .kind(CommandOptionType::Channel)
                        .required(true)
                })
                .create_option(|sub_command| {
                    sub_command
                        .name("message")
                        .description("The message which should be send.")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
        });

        commands
    }

    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        _: &sqlx::PgPool,
        _: &Configuration,
    ) -> Result<(), CommandError> {
        self.run(&command, ctx, &command.data.options).await
    }
}

impl SayCommand {
    async fn run(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        options: &[CommandDataOption],
    ) -> Result<(), CommandError> {
        command
            .create_interaction_response(ctx, |response| {
                response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
            })
            .await?;

        let channel_id = PositionalOptionParser::parse_channel_id(options, 0)?;
        let message = PositionalOptionParser::parse_string(options, 1)?;

        channel_id
            .send_message(&ctx.http, |create_message| create_message.content(message))
            .await?;

        command.delete_original_interaction_response(ctx).await?;

        Ok(())
    }
}
