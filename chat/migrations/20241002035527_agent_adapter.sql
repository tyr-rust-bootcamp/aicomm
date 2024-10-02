-- add adapter_type, adapter and model to agent
CREATE TYPE adapter_type AS ENUM(
    'openai',
    'ollama'
);

ALTER TABLE chat_agents
    ADD COLUMN adapter adapter_type NOT NULL DEFAULT 'ollama',
    ADD COLUMN model VARCHAR(255) NOT NULL DEFAULT 'llama3.2';
