CREATE TABLE IF NOT EXISTS events
(
    id          INTEGER   NOT NULL PRIMARY KEY,
    url         VARCHAR   NOT NULL,
    ip          VARCHAR   NOT NULL,
    user_agent  VARCHAR   NOT NULL,
    fingerprint VARCHAR   NOT NULL,
    is_private  BOOLEAN   NOT NULL,
    property_id VARCHAR   NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    FOREIGN KEY (property_id) REFERENCES properties (id)
);