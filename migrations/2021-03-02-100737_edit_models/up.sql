ALTER TABLE models RENAME TO tempmodels;

CREATE TABLE models
(
    id INTEGER PRIMARY KEY NOT NULL,
    filename TEXT UNIQUE NOT NULL,
    material TEXT DEFAULT NULL,
    texture TEXT DEFAULT NULL
);

INSERT INTO models
    (id, filename)
SELECT id, filename
FROM tempmodels;

DROP TABLE tempmodels;
