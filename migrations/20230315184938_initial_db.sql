-- Add migration script here

CREATE TABLE IF NOT EXISTS discord_user(
    discord_user_id BIGINT NOT NULL,
    discord_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    user_name VARCHAR(50),
    PRIMARY KEY (discord_user_id)
);

CREATE TABLE IF NOT EXISTS guild(
    discord_guild_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    initial_name VARCHAR(50) NOT NULL,
    welcome_channel_id BIGINT,
    command_log_channel_id BIGINT,
    infraction_log_channel_id BIGINT,
    PRIMARY KEY (discord_guild_id)
);

CREATE TABLE IF NOT EXISTS ticket(
    id_ticket SERIAL,
    discord_guild_id BIGINT NOT NULL,
    messages INTEGER NOT NULL,
    archived_messages TEXT,
    ticket_channel_id BIGINT NOT NULL,
    is_archived BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY (id_ticket),
    FOREIGN KEY (discord_guild_id) REFERENCES guild(discord_guild_id),
    FOREIGN KEY (create_user_id) REFERENCES discord_user(discord_user_id)
);

CREATE TABLE IF NOT EXISTS command_info(
    id_command_info SERIAL,
    discord_guild_id BIGINT NOT NULL,
    command_name VARCHAR(255) NOT NULL,
    parameter VARCHAR(255),
    create_user_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_command_info),
    FOREIGN KEY (discord_guild_id) REFERENCES guild(discord_guild_id),
    FOREIGN KEY (create_user_id) REFERENCES discord_user(discord_user_id)
);

CREATE TABLE IF NOT EXISTS infraction(
    id_infraction SERIAL,
    discord_guild_id BIGINT NOT NULL,
    punisher_id BIGINT NOT NULL,
    punished_id BIGINT NOT NULL,
    reason VARCHAR(255),
    create_date TIMESTAMP NOT NULL,
    punishment_type VARCHAR(50) NOT NULL,
    PRIMARY KEY (id_infraction),
    FOREIGN KEY (discord_guild_id) REFERENCES guild(discord_guild_id),
    FOREIGN KEY (punisher_id) REFERENCES discord_user(discord_user_id),
    FOREIGN KEY (punished_id) REFERENCES discord_user(discord_user_id)
);

CREATE TABLE IF NOT EXISTS message_activity_log(
    id_message_activity_log SERIAL,
    writer_id BIGINT NOT NULL,
    discord_guild_id BIGINT NOT NULL,
    write_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_message_activity_log),
    FOREIGN KEY (discord_guild_id) REFERENCES guild(discord_guild_id),
    FOREIGN KEY (writer_id) REFERENCES discord_user(discord_user_id)
);

CREATE TABLE IF NOT EXISTS emoji(
    id_emoji SERIAL,
    name VARCHAR(255),
    emoji_id BIGINT,
    create_user_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_emoji),
    FOREIGN KEY (create_user_id) REFERENCES discord_user(discord_user_id)
);

CREATE TABLE IF NOT EXISTS role(
    discord_role_id BIGINT NOT NULL,
    discord_guild_id BIGINT NOT NULL,
    is_join_role BOOLEAN NOT NULL,
    is_warning_role BOOLEAN NOT NULL,
    is_ticket_manager_role BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY (discord_role_id),
    FOREIGN KEY (discord_guild_id) REFERENCES guild(discord_guild_id),
    FOREIGN KEY (create_user_id) REFERENCES discord_user(discord_user_id)
);

CREATE TABLE IF NOT EXISTS role_right(
    id_role_right SERIAL,
    bot_right VARCHAR(50) NOT NULL,
    discord_role_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY(id_role_right),
    FOREIGN KEY (create_user_id) REFERENCES discord_user(discord_user_id),
    FOREIGN KEY (discord_role_id) REFERENCES role(discord_role_id)
);
