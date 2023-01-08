-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fd_file (
    inode INTEGER NOT NULL PRIMARY KEY, 

    locked BOOLEAN DEFAULT 0,
    pass_hash TEXT,

    nb_dl INTEGER DEFAULT 0
    );