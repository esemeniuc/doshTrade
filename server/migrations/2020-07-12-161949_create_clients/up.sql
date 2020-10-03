CREATE TABLE IF NOT EXISTS clients
(
    id         INTEGER   NOT NULL PRIMARY KEY,
    endpoint   VARCHAR   NOT NULL,
    p256dh     VARCHAR   NOT NULL,
    auth       VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (endpoint, p256dh, auth)
);