CREATE TABLE IF NOT EXISTS intraday_prices
(
    id        VARCHAR   NOT NULL PRIMARY KEY,
    stock_id  VARCHAR   NOT NULL,
    price     VARCHAR   NOT NULL,
    volume    INTERGER  NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);