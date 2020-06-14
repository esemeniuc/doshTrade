CREATE TABLE IF NOT EXISTS intraday_prices
(
    id        VARCHAR        NOT NULL PRIMARY KEY,
    stock_id  INTEGER        NOT NULL,
    price     DECIMAL(12, 2) NOT NULL,
    volume    INTEGER        NOT NULL,
    timestamp TIMESTAMP      NOT NULL,
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);