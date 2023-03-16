use chrono::NaiveDateTime;

pub struct RoleRight {
    id_role_right: i32,
    bot_right: BotRight,
    role_id: u64,
    guild_id: u64,
    create_date: NaiveDateTime,
    create_user_id: u64,
    modify_date: Option<NaiveDateTime>,
    modify_user_id: Option<u64>,
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