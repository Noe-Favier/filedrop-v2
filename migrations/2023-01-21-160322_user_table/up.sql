-- Your SQL goes here
CREATE TABLE fd_user (
    id INTEGER NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    pass_hash TEXT NOT NULL
);