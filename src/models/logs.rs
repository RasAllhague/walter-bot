use chrono::NaiveDateTime;

pub struct CommandInfo {
    id_command_info: i32,
    guild_id: u64,
    command_name: String,
    parameter: Option<String>,
    create_user_id: u64,
    create_date: NaiveDateTime,
}

pub struct Infraction {
    id_infraction: i32,
    guild_id: u64,
    punisher_id: u64,
    punished_id: u64,
    reason: Option<String>,
    create_date: NaiveDateTime,
    punishment_type: PunishmentType,
}

pub enum PunishmentType {
    Timeout,
    Warn,
    Kick,
    Ban,
}

pub struct MessageActivityLog {
    id_message_activity_log: i32,
    writer_id: u64,
    guild_id: u64,
    write_date: NaiveDateTime,
}
