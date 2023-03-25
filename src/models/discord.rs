use chrono::NaiveDateTime;

pub struct User {
    pub user_id: u64,
    pub guild_id: u64,
    pub user_name: Option<String>,
    pub create_date: NaiveDateTime,
}

pub struct Guild {
    pub guild_id: u64,
    pub create_date: NaiveDateTime,
    pub welcome_channel_id: Option<u64>,
    pub command_log_channel_id: Option<u64>,
    pub infraction_log_channel_id: Option<u64>,
}

pub struct Emoji {
    pub id_emoji: i32,
    pub guild_id: u64,
    pub name: Option<String>,
    pub emoji_id: Option<u64>,
    pub create_user_id: u64,
    pub create_date: NaiveDateTime,
}

pub struct Role {
    pub role_id: u64,
    pub guild_id: u64,
    pub is_join_role: bool,
    pub is_warning_role: bool,
    pub is_ticket_manager_role: bool,
    pub create_date: NaiveDateTime,
    pub create_user_id: u64,
    pub modify_date: Option<NaiveDateTime>,
    pub modify_user_id: Option<u64>,
}
