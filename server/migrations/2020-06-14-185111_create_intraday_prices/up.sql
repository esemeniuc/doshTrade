CREATE TABLE IF NOT EXISTS intraday_prices
(
    id        INTEGER        NOT NULL PRIMARY KEY,
    stock_id  INTEGER        NOT NULL,
    price     DECIMAL(12, 2) NOT NULL,
    volume    BIGINT         NOT NULL,
    timestamp TIMESTAMP      NOT NULL,
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);

CREATE INDEX index_intraday_prices_on_timestamp ON intraday_prices(timestamp);