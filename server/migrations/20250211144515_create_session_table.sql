CREATE TABLE if not exists session_table ( 
	id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	user_id UUID NOT NULL REFERENCES users,
    user_agent text,
	expired_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
);
