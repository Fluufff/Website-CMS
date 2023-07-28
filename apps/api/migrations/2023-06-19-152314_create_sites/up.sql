CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE sites (
	id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
	slug TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	url TEXT,
	image TEXT,
	description TEXT,
	deleted BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
