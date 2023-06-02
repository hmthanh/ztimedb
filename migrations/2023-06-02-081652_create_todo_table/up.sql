-- Your SQL goes here
CREATE TABLE todos (
    id VARCHAR(255) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
CREATE TABLE bandwidth (
    id VARCHAR(255) PRIMARY KEY,
    server_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);