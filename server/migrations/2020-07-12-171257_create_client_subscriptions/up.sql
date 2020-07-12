CREATE TABLE IF NOT EXISTS client_subscriptions
(
    client_subscription_id INTEGER NOT NULL PRIMARY KEY,
    client_id              INTEGER NOT NULL,
    stock_id               INTEGER NOT NULL,
    FOREIGN KEY (client_id) REFERENCES clients (id),
    FOREIGN KEY (stock_id) REFERENCES stocks (id)
);