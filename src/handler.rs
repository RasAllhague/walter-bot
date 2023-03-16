use std::{rc::Rc, sync::Arc};

use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, Ready, ResumedEvent, command::Command},
    prelude::{Context, EventHandler},
};
use tracing::{info, instrument, log::{debug, error}};

use crate::commands::{SlashCommand, infractions::InfractionCommand};

pub struct Handler {
    pub database: sqlx::PgPool,
    pub commands: Vec<Arc<dyn SlashCommand>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            debug!("Received command interaction: {:#?}", command);

            if command.guild_id.is_none() {
                return;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        
        let guild_commands = Command::create_global_application_command(&ctx.http, |command_builder| {
            for command in &self.commands {
                command.register(command_builder);
            }

            command_builder
        })
        .await;

        if let Err(why) = guild_commands {
            error!("Failed to create slash commands. {}", why);
        }
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}
