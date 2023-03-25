use chrono::NaiveDateTime;

pub struct User {
    user_id: u64,
    guild_id: u64,
    user_name: Option<String>,
    create_date: NaiveDateTime,
}

pub struct Guild {
    guild_id: u64,
    create_date: NaiveDateTime,
    welcome_channel_id: Option<u64>,
    command_log_channel_id: Option<u64>,
    infraction_log_channel_id: Option<u64>,
}

pub struct Emoji {
    id_emoji: i32,
    guild_id: u64,
    name: Option<String>,
    emoji_id: Option<u64>,
    create_user_id: u64,
    create_date: NaiveDateTime,
}

pub struct Role {
    role_id: u64,
    guild_id: u64,
    is_join_role: bool,
    is_warning_role: bool,
    is_ticket_manager_role: bool,
    create_date: NaiveDateTime,
    create_user_id: u64,
    modify_date: Option<NaiveDateTime>,
    modify_user_id: Option<u64>,
}