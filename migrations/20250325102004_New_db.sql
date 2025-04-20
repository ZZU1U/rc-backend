-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Add migration script here
CREATE TABLE car(  
    -- pk INT PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY UNIQUE,
    id UUID NOT NULL PRIMARY KEY,
    last_seen TIMESTAMPTZ NOT NULL DEFAULT now(),
    name TEXT NOT NULL UNIQUE,
    image_url TEXT,
    description TEXT,
    key_hash TEXT NOT NULL
);
COMMENT ON TABLE car IS 'car database';
COMMENT ON COLUMN car.name IS 'Car name, e.g. "Joe Peach car"';
COMMENT ON COLUMN car.description IS 'Additional description';

CREATE TABLE "user"(  
    -- pk INT PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY UNIQUE,
    id UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_super BOOLEAN DEFAULT false NOT NULL,
    is_verified BOOLEAN DEFAULT false NOT NULL
);
COMMENT ON TABLE "user" IS 'user database';
COMMENT ON COLUMN "user".email IS 'Email is used to send message when account gets verified or declined';
COMMENT ON COLUMN "user".password_hash IS 'Hashed user password';
COMMENT ON COLUMN "user".is_super IS 'If user has super rights';
COMMENT ON COLUMN "user".is_verified IS 'Is this a new request for account or real account';
