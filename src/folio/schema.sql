CREATE TABLE IF NOT EXISTS fills (
    fill_id TEXT PRIMARY KEY,
    kind TEXT,
    order_id INTEGER,
    market TEXT,
    quantity TEXT,
    price TEXT,
    dir TEXT,
    recv_time INTEGER,
    trade_time INTEGER,
    raw BLOB
);