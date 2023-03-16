pub mod infractions;
pub mod parser;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use self::parser::ParserError;

pub trait SlashCommand: Send + Sync {
    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        database: &sqlx::PgPool,
    ) -> Result<(), CommandError>;
}

#[derive(Debug)]
pub enum CommandError {
    Db(sqlx::Error),
    Parser(ParserError),
}
