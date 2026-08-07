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

CREATE SEQUENCE IF NOT EXISTS feature_scores START WITH 1 INCREMENT BY 1;

CREATE TABLE levels (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    name VARCHAR(20) NOT NULL,
    description VARCHAR(180) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,

    version INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    original_level_id INTEGER NOT NULL DEFAULT 0,
        
    length level_length NOT NULL,
    objects INTEGER NOT NULL CHECK (objects > 0),
    requested_stars SMALLINT NOT NULL CHECK (requested_stars BETWEEN 1 AND 10),
    stars SMALLINT NOT NULL DEFAULT 0 CHECK (stars BETWEEN 0 AND 10),
    coins SMALLINT NOT NULL DEFAULT 0 CHECK (coins BETWEEN 0 AND 3),
    likes INTEGER NOT NULL DEFAULT 0,
    dislikes INTEGER NOT NULL DEFAULT 0,
    downloads INTEGER NOT NULL DEFAULT 0,
    difficulty difficulty NOT NULL DEFAULT 'NA',
    demon_difficulty demon_difficulty NOT NULL DEFAULT 'Hard',

    is_rated BOOLEAN GENERATED ALWAYS AS (
        stars > 0
    ) STORED,

    is_featured BOOLEAN GENERATED ALWAYS AS (
        feature_score > 0
    ) STORED,
    
    feature_score INTEGER NOT NULL DEFAULT 0, 

    is_auto BOOLEAN NOT NULL DEFAULT FALSE,
    is_ldm BOOLEAN NOT NULL DEFAULT FALSE,
    is_two_player BOOLEAN NOT NULL DEFAULT FALSE,
    is_platformer BOOLEAN NOT NULL DEFAULT FALSE,
    is_gauntlet BOOLEAN NOT NULL DEFAULT FALSE,
    is_demon BOOLEAN NOT NULL DEFAULT FALSE,
    has_verified_coins BOOLEAN NOT NULL DEFAULT FALSE,
    
    official_song_id INTEGER NOT NULL,
    song_id INTEGER NOT NULL CHECK (song_id >= 0),

    visibility visibility NOT NULL,

    featured_at TIMESTAMPTZ DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE VIEW level_view AS
SELECT
    levels.*,
    users.username
FROM levels
JOIN users ON levels.user_id = users.id;
