use chrono::NaiveDateTime;

pub struct RoleSelect {
    pub id_role_select: i32,
    pub description: String,
    pub message_id: u64,
    pub channel_id: u64,
    pub guild_id: u64,
    pub create_date: NaiveDateTime,
    pub create_user_id: u64,
    pub modify_date: Option<NaiveDateTime>,
    pub modify_user_id: Option<u64>,
}

pub struct RoleSelectReaction {
    pub id_role_select_reaction: i32,
    pub message: String,
    pub emoji_id: u64,
    pub guild_id: u64,
    pub role_id: u64,
    pub role_select_id: i32,
    pub create_date: NaiveDateTime,
    pub create_user_id: u64,
    pub modify_date: Option<NaiveDateTime>,
    pub modify_user_id: Option<u64>,
}
