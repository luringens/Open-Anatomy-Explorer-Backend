ALTER TABLE models RENAME TO tempmodels;

CREATE TABLE models
(
    id INTEGER PRIMARY KEY NOT NULL,
    filename TEXT UNIQUE NOT NULL,
    material TEXT DEFAULT NULL,
    texture TEXT DEFAULT NULL,
    category TEXT DEFAULT NULL
);

INSERT INTO models
    (id, filename, material, texture)
SELECT id, filename, material, texture
FROM tempmodels;

DROP TABLE tempmodels;
