use serenity::{
    async_trait,
    model::prelude::{interaction::Interaction, Ready, ResumedEvent},
    prelude::{Context, EventHandler},
};
use tracing::{info, instrument, log::debug};

pub struct Handler {
    pub database: sqlx::PgPool,
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
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}
