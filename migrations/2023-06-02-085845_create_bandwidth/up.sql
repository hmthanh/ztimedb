-- Your SQL goes here

CREATE TABLE IF NOT EXISTS bandwidth (
    id VARCHAR(255) PRIMARY KEY,
    server_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);


