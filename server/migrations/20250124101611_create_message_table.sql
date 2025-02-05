-- Add migration script here
CREATE TABLE if not exists message (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    from_user UUID NOT NULL REFERENCES users,
    to_user UUID NOT NULL REFERENCES users,
    message TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);