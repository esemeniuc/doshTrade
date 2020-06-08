CREATE TABLE IF NOT EXISTS properties
(
    id           VARCHAR   NOT NULL PRIMARY KEY,
    website_name VARCHAR   NOT NULL,
    website_url  VARCHAR   NOT NULL,
    user_id      INTERGER  NOT NULL,
    created_at   TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);