CREATE TABLE IF NOT EXISTS servers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    server_id INTEGER NOT NULL UNIQUE
);

-- This table will always contain the ~BEGIN and ~END tokens
CREATE TABLE IF NOT EXISTS tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token TEXT NOT NULL UNIQUE,
    server_id INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
);

CREATE TABLE IF NOT EXISTS token_relationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    current_token_id INTEGER NOT NULL,
    related_token_id INTEGER NOT NULL,
    is_next BOOLEAN NOT NULL,
    weight INTEGER NOT NULL,
    FOREIGN KEY (current_token_id) REFERENCES tokens(id),
    FOREIGN KEY (related_token_id) REFERENCES tokens(id),
    UNIQUE (current_token_id, related_token_id, is_next)
);

CREATE TABLE IF NOT EXISTS channels (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    channel_id INTEGER NOT NULL UNIQUE,
    server_id INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
);

CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    message_id INTEGER NOT NULL,
    server_id INTEGER NOT NULL,
    channel_id INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id)
    FOREIGN KEY (channel_id) REFERENCES channels(id)
);