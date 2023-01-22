-- Your SQL goes here
CREATE TABLE fd_file (
    inode INTEGER NOT NULL PRIMARY KEY, 

    locked BOOLEAN DEFAULT 0 NOT NULL,
    pass_hash TEXT NOT NULL,

    nb_dl INTEGER DEFAULT 0 NOT NULL
    );