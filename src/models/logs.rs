use chrono::NaiveDateTime;

pub struct CommandInfo {
    pub id_command_info: i32,
    pub guild_id: u64,
    pub command_name: String,
    pub parameter: Option<String>,
    pub create_user_id: u64,
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
