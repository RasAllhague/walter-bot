use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::ApplicationCommandInteraction, command::CommandOptionType},
    prelude::Context,
};

use super::{CommandError, SlashCommand};

pub struct InfractionCommand;

impl InfractionCommand {
    fn build_warn_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("infraction")
            .description("Command group for punishing users.")
            .create_option(|sub_command| {
                sub_command
                    .name("warn") 
                    .description("Warns one or more user for a specified reason.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason why the warning was given.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user1")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user2")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user3")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user4")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user5")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user6")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user7")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user8")
                            .description("User who gets a warning.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
            })
    }

    fn build_kick_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("infraction")
            .description("Command group for punishing users.")
            .create_option(|sub_command| {
                sub_command
                    .name("kick") 
                    .description("Kick one or more user for a specified reason.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason why the kick was given.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user1")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user2")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user3")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user4")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user5")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user6")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user7")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user8")
                            .description("User who gets a kick.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
            })
    }

    fn build_ban_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("infraction")
            .description("Command group for punishing users.")
            .create_option(|sub_command| {
                sub_command
                    .name("ban") 
                    .description("Ban one or more user for a specified reason.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("reason")
                            .description("The reason why the ban was given.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user1")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user2")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user3")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user4")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user5")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user6")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user7")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
                    .create_sub_option(|option| {
                        option
                            .name("user8")
                            .description("User who gets a ban.")
                            .kind(CommandOptionType::User)
                            .required(false)
                    })
            })
    }
}

impl SlashCommand for InfractionCommand {
    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        Self::build_warn_command(command);
        Self::build_kick_command(command);
        Self::build_ban_command(command)
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
