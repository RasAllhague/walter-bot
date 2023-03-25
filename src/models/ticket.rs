use chrono::NaiveDateTime;

pub struct Ticket {
    pub id_ticket: i32,
    pub guild_id: u64,
    pub messages: i32,
    pub archived_messages: Option<String>,
    pub ticket_channel_id: u64,
    pub is_archived: bool,
    pub create_date: NaiveDateTime,
    pub create_user_id: u64,
    pub modify_date: Option<NaiveDateTime>,
    pub modify_user_id: Option<u64>,
}
