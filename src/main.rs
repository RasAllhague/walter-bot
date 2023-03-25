use std::{env, sync::Arc};

use commands::{
    infractions::InfractionCommand, lls::LssCommand, nuke::NukeCommand,
    role_selection::RoleSelectionCommand, say::SayCommand, ticket::TicketCommand,
    timeout::TimeoutCommand, SlashCommand,
};
use config::AppConfigurations;
use handler::BotHandler;
use serenity::prelude::*;
use tracing::{instrument, log::error};

mod commands;
mod handler;
pub mod models;
mod utils;
mod config;



#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfigurations::from_env();

    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let mut commands: Vec<Arc<dyn SlashCommand>> = Vec::new();
    commands.push(Arc::new(InfractionCommand));
    commands.push(Arc::new(TimeoutCommand));
    commands.push(Arc::new(NukeCommand));
    commands.push(Arc::new(SayCommand));
    commands.push(Arc::new(RoleSelectionCommand));
    commands.push(Arc::new(TicketCommand));
    commands.push(Arc::new(LssCommand));

    let intents = GatewayIntents::default();
    let mut client = Client::builder(config.bot_token, intents)
        .event_handler(BotHandler::new(database, &commands, &config.lls_file_path))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
