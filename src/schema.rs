pub static SQL: &'static str = "
DROP TABLE IF EXISTS task;

CREATE TABLE task (
    id              INTEGER PRIMARY KEY,
    created         INTEGER NOT NULL,
    text            TEXT NOT NULL
);";
