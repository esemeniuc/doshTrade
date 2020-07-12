CREATE TABLE IF NOT EXISTS clients
(
    id       INTEGER NOT NULL PRIMARY KEY,
    endpoint VARCHAR NOT NULL,
    p256dh   VARCHAR,
    auth     VARCHAR
);