use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOption},
                InteractionResponseType,
            },
            GuildId,
        },
        user::User,
    },
    prelude::Context,
};
use sqlx::PgPool;

use crate::handler::Configuration;

use super::{parser::OptionParser, CommandError, SlashCommand};

pub struct SayCommand;

#[async_trait]
impl SlashCommand for SayCommand {
    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| {
            command
                .name("say")
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
        match command.data.name.as_str() {
            "say" => self.run(&command, ctx, &command.data.options).await,
            _ => Ok(()),
        }
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

        let parser = OptionParser;

        let channel_id = parser.parse_channel_id(options, 0)?;
        let message = parser.parse_string(options, 1)?;

        channel_id
            .send_message(&ctx.http, |create_message| create_message.content(message))
            .await?;

        command.delete_original_interaction_response(ctx).await?;

        Ok(())
    }
}
