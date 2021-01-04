CREATE TABLE labels
(
    id INTEGER PRIMARY KEY NOT NULL,
    labelset INTEGER NOT NULL,
    name TEXT NOT NULL,
    colour TEXT NOT NULL,
    vertices BLOB NOT NULL
)
