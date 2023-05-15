use async_trait::async_trait;
use chrono::Utc;
use itertools::Itertools;
use serenity::{
    builder::CreateApplicationCommands,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
            }, UserId, Member,
        },
        Timestamp,
    },
    prelude::Context,
};
use tracing::log::{error, info};

use crate::handler::Configuration;

use super::{parser::PositionalOptionParser, CommandError, SlashCommand};

use tokio::time::Duration;

pub struct NukeCommand;

static COMMAND_NAME: &str = "nuke";

#[async_trait]
impl SlashCommand for NukeCommand {
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
                .description("Command for nuking an entire channel with a timeout nuke.")
                .create_option(|sub_command| {
                    sub_command
                        .name("minutes-in-past")
                        .description("Timeframe of messages.")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                })
                .create_option(|sub_command| {
                    sub_command
                        .name("nuclear-fallout-time")
                        .description("The duration of the nuclear fallout in minutes (timeout).")
                        .kind(CommandOptionType::Integer)
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
        if let Some(sender) = &command.member {
            if let Some(perms) = sender.permissions {
                if perms.administrator() {
                    run(command, ctx).await?;

                    return Ok(())
                }
            }
        }

        command
            .edit_original_interaction_response(ctx, |m| {
                m.content(format!("You have no permission to use the nuke command!"))
            })
            .await?;

        Ok(())
    }
}

pub async fn run(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<(), CommandError> {
    command
        .create_interaction_response(ctx, |m| {
            m.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await?;

    let minutes_in_past = PositionalOptionParser::parser_integer(&command.data.options, 0)?;
    let nuclear_fallout_time = PositionalOptionParser::parser_integer(&command.data.options, 1)?;

    display_timer(command, ctx).await?;

    command
            .edit_original_interaction_response(ctx, |m| {
                m.content(format!("Launching nuke:\nhttps://cdn.discordapp.com/attachments/886626413816479785/921707040693448714/FGNuke.gif"))
            })
            .await?;

    let now_in_past = Utc::now() - chrono::Duration::minutes(minutes_in_past);
    info!("now: {}", now_in_past);

    let messages = command
        .channel_id
        .messages(&ctx, |retriever| retriever.limit(500))
        .await?;
    let targeted_users: Vec<UserId> = messages
        .iter()
        .filter(|x| x.timestamp.naive_utc() > now_in_past.naive_utc())
        .unique_by(|x| x.author.id.0)
        .filter(|x| !x.author.bot)
        .map(|x| x.author.id)
        .collect();

    info!("Found targets: {:?}", targeted_users);

    let nuked_people = timeout_members(
        &targeted_users,
        command,
        ctx,
        nuclear_fallout_time,
    )
    .await;

    if nuked_people.len() != 0 {
        command
            .channel_id
            .send_message(&ctx.http, |create_message| {
                create_message.content("The following members where hit by the nuke:\n https://giphy.com/gifs/h5Dnf37npfhYQhfFLK")
            })
            .await?;

        for nuked_member in nuked_people {
            command
            .channel_id
            .send_message(&ctx.http, |create_message| {
                create_message.content(format!("<@{}>", nuked_member.user.id.0))
            })
            .await?;
        }
    } else {
        command
            .channel_id
            .send_message(&ctx.http, |create_message| {
                create_message.content("No members were hit by the nuke!")
            })
            .await?;
    }

    Ok(())
}

async fn display_timer(command: &ApplicationCommandInteraction, ctx: &Context) -> Result<(), CommandError> {
    Ok(for i in (0..11).rev() {
        command
            .edit_original_interaction_response(ctx, |m| {
                m.content(format!("Nuke will be launched in: {i} seconds."))
            })
            .await?;

        tokio::time::sleep(Duration::from_secs(1)).await;
    })
}

async fn timeout_members(
    targeted_users: &[UserId],
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    nuclear_fallout_time: i64,
) -> Vec<Member> {
    let mut nuked_people = Vec::new();

    for targeted_user in targeted_users {
        if let Ok(mut target_member) = command.guild_id.unwrap().member(&ctx, targeted_user).await {
            let now = Utc::now() + chrono::Duration::minutes(nuclear_fallout_time);

            match target_member
                .disable_communication_until_datetime(ctx, Timestamp::from(now))
                .await
            {
                Ok(_) => nuked_people.push(target_member.clone()),
                Err(why) => error!("Error while trying to timeout a member {target_member}: {why}"),
            };
        }
    }

    nuked_people
}
