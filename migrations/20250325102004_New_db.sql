-- Add migration script here
CREATE TABLE car(  
    pk INT PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY UNIQUE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    create_time TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen TIMESTAMPTZ NOT NULL DEFAULT now(),
    name TEXT NOT NULL UNIQUE,
    image_url TEXT,
    description TEXT,
    key_hash TEXT NOT NULL,
    creator_id UUID NOT NULL
);
COMMENT ON TABLE car IS 'car database';
COMMENT ON COLUMN car.name IS 'Car name, e.g. "Joe Peach car"';
COMMENT ON COLUMN car.description IS 'Additional description';
COMMENT ON COLUMN car.creator_id IS 'ID reference to "user" who created this car. User must be super';
--
CREATE TABLE "user"(  
    pk INT PRIMARY KEY NOT NULL GENERATED ALWAYS AS IDENTITY UNIQUE,
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    create_time TIMESTAMPTZ NOT NULL DEFAULT now(),
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    is_super BOOLEAN DEFAULT false NOT NULL
);
COMMENT ON TABLE "user" IS 'user database';
COMMENT ON COLUMN "user".username IS 'Username shown to others and used to login';
COMMENT ON COLUMN "user".password_hash IS 'Hashed user password';
COMMENT ON COLUMN "user".is_super IS 'If user has super rights';
