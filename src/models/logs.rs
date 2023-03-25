use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CommandInfo {
    pub id_command_info: i32,
    pub guild_id: i64,
    pub command_name: String,
    pub parameter: Option<String>,
    pub create_user_id: i64,
    pub create_date: NaiveDateTime,
}

pub struct Infraction {
    pub id_infraction: i32,
    pub guild_id: u64,
    pub punisher_id: u64,
    pub punished_id: u64,
    pub reason: Option<String>,
    pub create_date: NaiveDateTime,
    pub punishment_type: PunishmentType,
}

pub enum PunishmentType {
    Timeout,
    Warn,
    Kick,
    Ban,
}

pub struct MessageActivityLog {
    pub id_message_activity_log: i32,
    pub writer_id: u64,
    pub guild_id: u64,
    pub write_date: NaiveDateTime,
}

impl CommandInfo {
    pub fn new(guild_id: i64, command_name: &str, parameter: Option<String>, create_user: i64, create_date: NaiveDateTime) -> CommandInfo {
        CommandInfo {
            id_command_info: 0,
            guild_id,
            command_name: String::from(command_name),
            parameter,
            create_user_id: create_user,
            create_date
        }
    }

    pub async fn insert(&self, db: &PgPool) -> Result<CommandInfo, sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO command_infos
                (guild_id, command_name, parameter, create_user_id, create_date)
                VALUES
                ($1, $2, $3, $4, $5)
                RETURNING id_command_info;",
            self.guild_id,
            self.command_name,
            self.parameter,
            self.create_user_id,
            self.create_date
        )
        .fetch_one(db)
        .await?
        .id_command_info;

        let mut new_command_info = self.clone();
        new_command_info.id_command_info = id; 

        return Ok(new_command_info);
    }
}