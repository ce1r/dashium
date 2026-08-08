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
    version INT NOT NULL DEFAULT 1 CHECK (version >= 1),
    original_level_id INTEGER NOT NULL DEFAULT 0,
    length level_length NOT NULL,
    objects INTEGER NOT NULL CHECK (objects > 0),
    requested_stars SMALLINT NOT NULL CHECK (requested_stars BETWEEN 1 AND 10),
    coins SMALLINT NOT NULL CHECK (coins BETWEEN 0 AND 3),
    likes INTEGER NOT NULL DEFAULT 0,
    dislikes INTEGER NOT NULL DEFAULT 0,
    downloads INTEGER NOT NULL DEFAULT 0,
    is_ldm BOOLEAN NOT NULL DEFAULT FALSE,
    is_two_player BOOLEAN NOT NULL DEFAULT FALSE,
    is_platformer BOOLEAN NOT NULL DEFAULT FALSE,
    official_song_id INTEGER NOT NULL,
    song_id INTEGER NOT NULL CHECK (song_id >= 0),
    visibility visibility NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

DROP TYPE IF EXISTS rating;
CREATE TYPE rating AS ENUM (
    'Star',
    'Feature',
    'Epic',
    'Legendary',
    'Mythic'
);

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

CREATE TABLE ratings (
    level_id INTEGER PRIMARY KEY REFERENCES levels(id) ON DELETE CASCADE,
    rating rating NOT NULL,
    demon_difficulty demon_difficulty DEFAULT NULL,
    difficulty difficulty NOT NULL DEFAULT 'NA',
    has_verified_coins BOOLEAN NOT NULL DEFAULT FALSE,
    stars SMALLINT NOT NULL CHECK (stars BETWEEN 1 AND 10),

    is_auto BOOLEAN GENERATED ALWAYS AS (stars = 1) STORED,
    is_demon BOOLEAN GENERATED ALWAYS AS (difficulty = 'Demon') STORED,
    is_featured BOOLEAN GENERATED ALWAYS AS (rating IN ('Feature', 'Epic', 'Legendary', 'Mythic')) STORED,

    rated_by INTEGER DEFAULT NULL REFERENCES users(id) ON DELETE CASCADE,
    rated_at TIMESTAMPTZ DEFAULT NULL
);

CREATE VIEW level_view AS
SELECT
    l.*,
    u.username,
    r.rating,
    COALESCE(r.stars, 0) AS stars,
    COALESCE(r.difficulty, 'NA'::difficulty) AS difficulty,
    r.demon_difficulty,
    COALESCE(r.has_verified_coins, false) AS has_verified_coins,
    COALESCE(r.is_auto, false) AS is_auto,
    COALESCE(r.is_demon, false) AS is_demon,
    COALESCE(r.is_featured, false) AS is_featured,
    (r.level_id IS NOT NULL) AS is_rated,
    r.rated_by,
    r.rated_at
FROM levels l
JOIN users u ON l.user_id = u.id
LEFT JOIN ratings r ON l.id = r.level_id;
