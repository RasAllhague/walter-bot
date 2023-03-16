use chrono::NaiveDateTime;

pub struct RoleSelect {
    id_role_select: i32,
    description: String,
    message_id: u64,
    channel_id: u64,
    guild_id: u64,
    create_date: NaiveDateTime,
    create_user_id: u64,
    modify_date: Option<NaiveDateTime>,
    modify_user_id: Option<u64>,
}

pub struct RoleSelectReaction {
    id_role_select_reaction: i32,
    message: String,
    emoji_id: u64,
    guild_id: u64,
    role_id: u64,
    role_select_id: i32,
    create_date: NaiveDateTime,
    create_user_id: u64,
    modify_date: Option<NaiveDateTime>,
    modify_user_id: Option<u64>,
}
