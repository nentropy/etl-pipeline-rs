-- Create sessions table
CREATE TABLE sessions (
    session_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    created TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    duration INTERVAL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Create users table
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    created TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Create logs table (example table for logging session activities)
CREATE TABLE logs (
    log_id SERIAL PRIMARY KEY,
    session_id INT NOT NULL,
    log_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    log_message TEXT,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id) ON DELETE CASCADE
);

-- Create data_entries table (example table for storing data entries associated with sessions)
CREATE TABLE data_entries (
    entry_id SERIAL PRIMARY KEY,
    session_id INT NOT NULL,
    data_key VARCHAR(100) NOT NULL,
    data_value TEXT,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id) ON DELETE CASCADE
);

-- Create errors table (example table for capturing errors during ETL processes)
CREATE TABLE errors (
    error_id SERIAL PRIMARY KEY,
    session_id INT NOT NULL,
    error_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    error_message TEXT,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id) ON DELETE CASCADE
);

-- Create indexes for efficient querying
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_logs_session_id ON logs(session_id);
CREATE INDEX idx_data_entries_session_id ON data_entries(session_id);
CREATE INDEX idx_errors_session_id ON errors(session_id);
