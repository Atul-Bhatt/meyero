-- Add migration script here
CREATE TABLE if not exists token ( 
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	user_id UUID NOT NULL REFERENCES users(id),
	token TEXT NOT NULL,
	expires_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW() + INTERVAL '1 hours',
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	revoked TIMESTAMP
);
