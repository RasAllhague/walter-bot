use std::{env, sync::Arc};

use commands::{SlashCommand, infractions::InfractionCommand, timeout::TimeoutCommand, nuke::NukeCommand, say::SayCommand, role_selection::RoleSelectionCommand, ticket::TicketCommand, lls::LssCommand};
use handler::Handler;
use serenity::prelude::*;
use tracing::{instrument, log::error};

mod commands;
mod handler;
pub mod models;
mod utils;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("WALTER_BOT_TOKEN").expect("Expected a token in the environment");
    let db_url = env::var("WALTER_DATABASE_URL").expect("Expected database url in the environment");

    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let mut commands: Vec<Arc<dyn SlashCommand>> = Vec::new();
    // commands.push(Arc::new(InfractionCommand));
    // commands.push(Arc::new(TimeoutCommand));
    commands.push(Arc::new(NukeCommand));
    commands.push(Arc::new(SayCommand));
    // commands.push(Arc::new(RoleSelectionCommand));
    // commands.push(Arc::new(TicketCommand));
    commands.push(Arc::new(LssCommand));

    let intents = GatewayIntents::default();
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { database, commands })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
