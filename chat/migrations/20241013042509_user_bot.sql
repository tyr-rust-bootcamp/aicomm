-- add is_bot column to users table
ALTER TABLE users
    ADD COLUMN is_bot BOOLEAN NOT NULL DEFAULT FALSE;
