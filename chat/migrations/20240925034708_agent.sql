-- Add migration script here

-- modify chat table to add agents column
ALTER TABLE chats ADD COLUMN agents bigint[] NOT NULL DEFAULT '{}';

-- modify messages table to add 'modified_content' column
ALTER TABLE messages ADD COLUMN modified_content TEXT;

-- add agent_type type
CREATE TYPE agent_type AS ENUM ('proxy', 'reply', 'tap');

-- add chat_agent table
CREATE TABLE IF NOT EXISTS chat_agents(
  id BIGSERIAL PRIMARY KEY,
  chat_id BIGINT NOT NULL REFERENCES chats(id),
  name TEXT NOT NULL UNIQUE,
  type agent_type NOT NULL DEFAULT 'reply',
  prompt TEXT NOT NULL,
  args JSONB NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
