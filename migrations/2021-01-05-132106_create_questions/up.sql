CREATE TABLE questions
(
    id INTEGER PRIMARY KEY NOT NULL,
    quiz INTEGER NOT NULL,
    questiontype SMALLINT NOT NULL,
    textprompt TEXT NOT NULL,
    textanswer TEXT,
    label INTEGER,
    showregions SMALLINT NOT NULL
);
