use std::path::PathBuf;

use async_trait::async_trait;
use rand::Rng;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};
use tokio::fs::read_dir;
use tracing::log::info;

use crate::handler::Configuration;

use super::{CommandError, SlashCommand};

pub struct LssCommand;

static COMMAND_NAME: &str = "lls";

async fn get_random_file(lls_image_path: &str) -> Result<PathBuf, CommandError> {
    let mut dir = read_dir(lls_image_path).await?;
    let mut paths = Vec::new();

    while let Some(child) = dir.next_entry().await? {
        if child.metadata().await?.is_file() {
            paths.push(child.path());
        }
    }

    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..paths.len());

    Ok(paths[index].clone())
}

#[async_trait]
impl SlashCommand for LssCommand {
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
        _: &sqlx::PgPool,
        configuration: &Configuration,
    ) -> Result<(), CommandError> {
        command
            .create_interaction_response(ctx, |m| {
                m.kind(InteractionResponseType::DeferredChannelMessageWithSource)
            })
            .await?;

        let image_path = get_random_file(&configuration.lls_file_path).await?;
        info!("Sending lls image: {:?}", image_path);

        command.edit_original_interaction_response(ctx, |m| m.content("Long live Stalweidism!"))
            .await?;
        command.channel_id
            .send_message(&ctx.http, |create_message| create_message.add_file(&image_path))
            .await?;

        Ok(())
    }

    fn name(&self) -> String {
        String::from(COMMAND_NAME)
    }
}

impl LssCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(COMMAND_NAME)
            .description("Command for sending lls images.")
    }
}
