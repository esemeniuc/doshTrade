CREATE TABLE IF NOT EXISTS users
(
    id                SERIAL PRIMARY KEY,
    first_name        VARCHAR   NOT NULL,
    last_name         VARCHAR   NOT NULL,
    email             VARCHAR   NOT NULL UNIQUE,
    password          VARCHAR   NOT NULL,
    auth_bearer_token VARCHAR   NOT NULL,
    created_at        TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS stocks
(
    id     SERIAL PRIMARY KEY,
    ticker VARCHAR NOT NULL UNIQUE,
    name   VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS intraday_prices
(
    id        SERIAL PRIMARY KEY,
    stock_id  INTEGER          NOT NULL,
    price     DOUBLE PRECISION NOT NULL, --TODO: change to integer
    volume    BIGINT           NOT NULL,
    timestamp TIMESTAMP        NOT NULL,
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);

CREATE INDEX index_intraday_prices_on_timestamp ON intraday_prices (timestamp);

CREATE TABLE IF NOT EXISTS clients
(
    id         SERIAL PRIMARY KEY,
    endpoint   VARCHAR   NOT NULL,
    p256dh     VARCHAR   NOT NULL,
    auth       VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (endpoint, p256dh, auth)
);

CREATE TABLE IF NOT EXISTS client_subscriptions
(
    id         SERIAL PRIMARY KEY,
    client_id  INTEGER   NOT NULL,
    stock_id   INTEGER   NOT NULL,
    last_sent  TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients (id),
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);
CREATE INDEX index_client_subscriptions_on_client_id ON client_subscriptions (client_id);
CREATE INDEX index_client_subscriptions_on_stock_id ON client_subscriptions (stock_id);

CREATE TYPE OPTION_TYPE AS ENUM ('call','put');
CREATE TABLE IF NOT EXISTS option_quotes
(
    id          SERIAL PRIMARY KEY,
    string_id   VARCHAR          NOT NULL, --id from think or swim that has ticker, expiration, type, strike
    stock_id    INTEGER          NOT NULL,
    option_type OPTION_TYPE      NOT NULL,
    strike      DOUBLE PRECISION NOT NULL,
    expiration  TIMESTAMP        NOT NULL,
    bid         DOUBLE PRECISION NOT NULL,
    ask         DOUBLE PRECISION NOT NULL,
    last        DOUBLE PRECISION NOT NULL,
    delta       DOUBLE PRECISION,
    gamma       DOUBLE PRECISION,
    theta       DOUBLE PRECISION,
    vega        DOUBLE PRECISION,
    rho         DOUBLE PRECISION,
    volatility  DOUBLE PRECISION,
    time_value  DOUBLE PRECISION,
    created_at  TIMESTAMP        NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);

CREATE INDEX index_option_quotes_on_stock_id ON option_quotes (stock_id);
CREATE INDEX index_option_quotes_on_expiration ON option_quotes (expiration);