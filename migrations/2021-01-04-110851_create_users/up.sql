CREATE TABLE users
(
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password BLOB NOT NULL,
    privilege INTEGER NOT NULL DEFAULT 0
)
