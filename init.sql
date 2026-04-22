CREATE TABLE IF NOT EXISTS users (
    id         SERIAL PRIMARY KEY,
    username   TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS messages (
    id         SERIAL PRIMARY KEY,
    username   TEXT NOT NULL,
    content    TEXT NOT NULL,
    sent_at    TIMESTAMP DEFAULT NOW()
);