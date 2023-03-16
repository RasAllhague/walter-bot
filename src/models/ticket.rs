use chrono::NaiveDateTime;

pub struct Ticket {
    id_ticket: i32,
    guild_id: u64,
    messages: i32,
    archived_messages: Option<String>,
    ticket_channel_id: u64,
    is_archived: bool,
    create_date: NaiveDateTime,
    create_user_id: u64,
    modify_date: Option<NaiveDateTime>,
    modify_user_id: Option<u64>,
}