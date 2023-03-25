use chrono::NaiveDateTime;

pub struct User {
    pub user_id: i64,
    pub guild_id: i64,
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

impl User {
    pub fn new(user_id: i64, guild_id: i64, user_name: Option<String>, create_date: NaiveDateTime) -> User {
        User {
            user_id,
            guild_id,
            user_name,
            create_date,
        }
    } 

    pub async fn get(db: &sqlx::PgPool, user_id: i64, guild_id: i64) -> Result<Option<User>, sqlx::Error> {
        let user: Option<User> = sqlx::query_as!(
            User,
            "SELECT user_id, guild_id, user_name, create_date 
                FROM users
                WHERE user_id = $1
                AND guild_id = $2;",
            user_id,
            guild_id,
        )
        .fetch_all(db)
        .await?
        .into_iter()
        .nth(0);

        Ok(user)
    }

    pub async fn exists(&self, db: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
        Ok(Self::get(db, self.user_id, self.guild_id).await?.is_some())
    }

    pub async fn insert(self, db: &sqlx::PgPool) -> Result<User, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users
                (guild_id, user_id, user_name, create_date)
                VALUES
                ($1, $2, $3, $4);",
            self.guild_id,
            self.user_id,
            self.user_name,
            self.create_date
        )
        .execute(db)
        .await?;

        return Ok(self);
    }
}