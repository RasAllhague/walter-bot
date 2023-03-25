use chrono::NaiveDateTime;

pub struct RoleRight {
    pub id_role_right: i32,
    pub bot_right: BotRight,
    pub role_id: u64,
    pub guild_id: u64,
    pub create_date: NaiveDateTime,
    pub create_user_id: u64,
    pub modify_date: Option<NaiveDateTime>,
    pub modify_user_id: Option<u64>,
}

pub enum BotRight {
    ManageRoleSelect,
    ManageRights,
    ManageRoles,
    UseActivity,
    UploadImages,
    ManageMembers,
    ManageTickets,
    Admin,
}
