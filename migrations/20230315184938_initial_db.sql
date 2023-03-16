-- Add migration script here

CREATE TABLE IF NOT EXISTS users(
    user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    user_name VARCHAR(50),
    PRIMARY KEY (user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS guilds(
    guild_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    initial_name VARCHAR(50) NOT NULL,
    welcome_channel_id BIGINT,
    command_log_channel_id BIGINT,
    infraction_log_channel_id BIGINT,
    PRIMARY KEY (guild_id)
);

CREATE TABLE IF NOT EXISTS tickets(
    id_ticket SERIAL,
    guild_id BIGINT NOT NULL,
    messages INTEGER NOT NULL,
    archived_messages TEXT,
    ticket_channel_id BIGINT NOT NULL,
    is_archived BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY (id_ticket),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS command_infos(
    id_command_info SERIAL,
    guild_id BIGINT NOT NULL,
    command_name VARCHAR(255) NOT NULL,
    parameter VARCHAR(255),
    create_user_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_command_info),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS infractions(
    id_infraction SERIAL,
    guild_id BIGINT NOT NULL,
    punisher_id BIGINT NOT NULL,
    punished_id BIGINT NOT NULL,
    reason VARCHAR(255),
    create_date TIMESTAMP NOT NULL,
    punishment_type VARCHAR(50) NOT NULL,
    PRIMARY KEY (id_infraction),
    FOREIGN KEY (punisher_id, guild_id) REFERENCES users(user_id, guild_id),
    FOREIGN KEY (punished_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS message_activity_logs(
    id_message_activity_log SERIAL,
    writer_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    write_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_message_activity_log),
    FOREIGN KEY (writer_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS emojis(
    id_emoji SERIAL,
    guild_id BIGINT NOT NULL,
    name VARCHAR(255),
    emoji_id BIGINT,
    create_user_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_emoji),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS roles(
    role_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    is_join_role BOOLEAN NOT NULL,
    is_warning_role BOOLEAN NOT NULL,
    is_ticket_manager_role BOOLEAN NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY (role_id, guild_id),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS role_rights(
    id_role_right SERIAL,
    bot_right VARCHAR(50) NOT NULL,
    role_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY(id_role_right),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS role_selects(
    id_role_select SERIAL,
    description VARCHAR(500) NOT NULL,
    message_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY(id_role_select),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS role_select_reaction(
    id_role_select_reaction SERIAL,
    message VARCHAR(255) NOT NULL,
    emoji_id BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    role_id INTEGER NOT NULL,
    role_select_id BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    create_user_id BIGINT NOT NULL,
    modify_date TIMESTAMP,
    modify_user_id BIGINT,
    PRIMARY KEY (id_role_select_reaction),
    FOREIGN KEY (emoji_id) REFERENCES emojis(id_emoji),
    FOREIGN KEY (role_select_id) REFERENCES role_selects(id_role_select),
    FOREIGN KEY (create_user_id, guild_id) REFERENCES users(user_id, guild_id)
);
