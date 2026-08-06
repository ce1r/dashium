DROP TYPE IF EXISTS demon_difficulty;
CREATE TYPE demon_difficulty AS ENUM (
    'Easy',
    'Medium',
    'Hard',
    'Insane',
    'Extreme'
);

DROP TYPE IF EXISTS difficulty;
CREATE TYPE difficulty AS ENUM (
    'NA',
    'Auto',
    'Easy',
    'Normal',
    'Hard',
    'Harder',
    'Insane',
    'Demon'
);

DROP TYPE IF EXISTS level_length;
CREATE TYPE level_length AS ENUM (
    'Tiny',
    'Short',
    'Medium',
    'Long',
    'XL'
);

DROP TYPE IF EXISTS visibility;
CREATE TYPE visibility AS ENUM (
    'Public',
    'FriendsOnly',
    'Private'
);


CREATE TABLE levels (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    name VARCHAR(20) NOT NULL,
    description VARCHAR(180) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_data TEXT NOT NULL,

    version INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    original_level_id INTEGER NOT NULL DEFAULT 0,
        
    length level_length NOT NULL,
    objects INTEGER NOT NULL CHECK (objects > 0),
    requested_stars SMALLINT NOT NULL CHECK (requested_stars BETWEEN 1 AND 10),
    coins SMALLINT NOT NULL CHECK (coins BETWEEN 0 AND 3),

    is_auto BOOLEAN NOT NULL,
    is_ldm BOOLEAN NOT NULL,
    is_two_player BOOLEAN NOT NULL,
    is_platformer BOOLEAN NOT NULL,
    is_gauntlet BOOLEAN NOT NULL,
    
    official_song_id INTEGER NOT NULL,
    song_id INTEGER NOT NULL CHECK (song_id >= 0),

    visibility visibility NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
