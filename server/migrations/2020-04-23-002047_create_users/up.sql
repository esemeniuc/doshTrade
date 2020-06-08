CREATE TABLE IF NOT EXISTS users
(
    id                INTEGER   NOT NULL PRIMARY KEY,
    first_name        VARCHAR   NOT NULL,
    last_name         VARCHAR   NOT NULL,
    email             VARCHAR   NOT NULL UNIQUE,
    password          VARCHAR   NOT NULL,
    auth_bearer_token VARCHAR   NOT NULL,
    created_at        TIMESTAMP NOT NULL
);