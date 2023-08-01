CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	source TEXT NOT NULL DEFAULT 'local',
	password Text NOT NULL,
	bio TEXT,
	avatar TEXT,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
