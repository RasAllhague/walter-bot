pub mod infractions;
pub mod lls;
pub mod nuke;
pub mod parser;
pub mod role_selection;
pub mod say;
pub mod ticket;
pub mod timeout;

use std::{error::Error, fmt};

use async_trait::async_trait;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::handler::Configuration;

use self::parser::ParserError;

#[async_trait]
pub trait SlashCommand: Send + Sync {
    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands;
    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        database: &sqlx::PgPool,
        configuration: &Configuration,
    ) -> Result<(), CommandError>;
}

#[derive(Debug)]
pub enum CommandError {
    Db(sqlx::Error),
    Parser(ParserError),
    Serenity(serenity::Error),
    IO(std::io::Error),
}

impl From<sqlx::Error> for CommandError {
    fn from(value: sqlx::Error) -> Self {
        CommandError::Db(value)
    }
}

impl From<ParserError> for CommandError {
    fn from(value: ParserError) -> Self {
        CommandError::Parser(value)
    }
}

impl From<serenity::Error> for CommandError {
    fn from(value: serenity::Error) -> Self {
        CommandError::Serenity(value)
    }
}

impl From<std::io::Error> for CommandError {
    fn from(value: std::io::Error) -> Self {
        CommandError::IO(value)
    }
}
