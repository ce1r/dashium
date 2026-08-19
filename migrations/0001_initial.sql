CREATE TYPE role AS ENUM (
    'User',
    'Moderator',
    'ElderModerator',
    'LeaderboardModerator',
    'Administrator'
);

CREATE TYPE message_setting AS ENUM (
    'All',
    'FriendsOnly',
    'None'
);

CREATE TYPE comment_setting AS ENUM (
    'All',
    'FriendsOnly',
    'None'
);

CREATE TYPE level_length AS ENUM (
    'Tiny',
    'Short',
    'Medium',
    'Long',
    'XL'
);

CREATE TYPE visibility AS ENUM (
    'Public',
    'FriendsOnly',
    'Private'
);

CREATE TYPE rating AS ENUM (
    'Star',
    'Feature',
    'Epic',
    'Legendary',
    'Mythic'
);

CREATE TYPE demon_difficulty AS ENUM (
    'Easy',
    'Medium',
    'Hard',
    'Insane',
    'Extreme'
);

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

CREATE TYPE item_type AS ENUM (
    'Orbs',
    'Coins',
    'Stars'
);

CREATE TABLE users (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    username VARCHAR(32) NOT NULL CONSTRAINT unique_username UNIQUE,
    email VARCHAR(255) NOT NULL CONSTRAINT unique_email UNIQUE,
    hash BYTEA NOT NULL,
    salt BYTEA NOT NULL,
    is_activated BOOLEAN NOT NULL DEFAULT TRUE,
    role role NOT NULL DEFAULT 'User',
    udid TEXT NOT NULL DEFAULT '',
    stars INTEGER NOT NULL DEFAULT 0,
    demons INTEGER NOT NULL DEFAULT 0,
    creator_points INTEGER NOT NULL DEFAULT 0,
    diamonds INTEGER NOT NULL DEFAULT 0,
    moons INTEGER NOT NULL DEFAULT 0,
    secret_coins INTEGER NOT NULL DEFAULT 0,
    user_coins INTEGER NOT NULL DEFAULT 0,
    cube SMALLINT NOT NULL DEFAULT 1,
    ship SMALLINT NOT NULL DEFAULT 1,
    ball SMALLINT NOT NULL DEFAULT 1,
    ufo SMALLINT NOT NULL DEFAULT 1,
    wave SMALLINT NOT NULL DEFAULT 1,
    robot SMALLINT NOT NULL DEFAULT 1,
    spider SMALLINT NOT NULL DEFAULT 1,
    swing SMALLINT NOT NULL DEFAULT 1,
    jetpack SMALLINT NOT NULL DEFAULT 1,
    glow SMALLINT NOT NULL DEFAULT 0,
    explosion SMALLINT NOT NULL DEFAULT 1,
    icon SMALLINT NOT NULL DEFAULT 1,
    icon_type SMALLINT NOT NULL DEFAULT 0,
    color1 SMALLINT NOT NULL DEFAULT 0,
    color2 SMALLINT NOT NULL DEFAULT 3,
    color3 SMALLINT NOT NULL DEFAULT -1,
    accept_friend_requests BOOLEAN NOT NULL DEFAULT TRUE,
    message_setting message_setting NOT NULL DEFAULT 'All',
    comment_setting comment_setting NOT NULL DEFAULT 'All',
    youtube VARCHAR(255) NOT NULL DEFAULT '',
    twitter VARCHAR(255) NOT NULL DEFAULT '',
    twitch VARCHAR(255) NOT NULL DEFAULT '',
    discord VARCHAR(255) NOT NULL DEFAULT '',
    instagram VARCHAR(255) NOT NULL DEFAULT '',
    tiktok VARCHAR(255) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    body VARCHAR(140) NOT NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
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

CREATE TABLE rates (
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

CREATE TABLE completions (
    level_id INTEGER NOT NULL REFERENCES levels (id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (level_id, user_id)
);

CREATE TABLE blocks (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    target_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_block UNIQUE (user_id, target_id),
    CONSTRAINT no_self_block CHECK (user_id != target_id)
);

CREATE TABLE messages (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    target_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    subject VARCHAR(255) NOT NULL,
    body VARCHAR(512) NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT no_self_message CHECK (user_id != target_id)
);

CREATE TABLE quests (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    item_type item_type NOT NULL,
    amount SMALLINT NOT NULL CHECK (amount > 0),
    reward SMALLINT NOT NULL CHECK (reward > 0),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO quests (item_type, amount, reward, name)
VALUES ('Orbs', 2000, 30, 'Gather 2000 Orbs'),
    ('Coins', 5, 20, 'Earn 5 Coins'),
    ('Stars', 10, 10, 'Collect 10 Stars');

CREATE TABLE friend_requests (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    target_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    body VARCHAR(255) NOT NULL,
    is_new BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_friend_request UNIQUE (user_id, target_id),
    CONSTRAINT no_self_friend_request CHECK (user_id != target_id)
);

CREATE TABLE friendships (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user1 INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    user2 INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    is_new1 BOOLEAN NOT NULL DEFAULT TRUE,
    is_new2 BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_friendship UNIQUE (user1, user2),
    CONSTRAINT no_self_friend CHECK (user1 != user2)
);

CREATE TABLE comments (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_id INTEGER NOT NULL REFERENCES levels (id) ON DELETE CASCADE,
    body VARCHAR(100) NOT NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    is_spam BOOLEAN NOT NULL DEFAULT FALSE,
    percent SMALLINT NOT NULL DEFAULT 0 CHECK (percent BETWEEN 0 AND 100),
    chat_color VARCHAR(11) NOT NULL DEFAULT '255,255,255',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE lists (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(25) NOT NULL,
    description VARCHAR(300) NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    downloads INTEGER NOT NULL DEFAULT 0,
    likes INTEGER NOT NULL DEFAULT 0,
    difficulty SMALLINT NOT NULL,
    rated BOOLEAN NOT NULL DEFAULT FALSE,
    levels INTEGER[] NOT NULL,
    reward INTEGER NOT NULL DEFAULT 0,
    requirement INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_stats()
RETURNS TRIGGER AS $$
DECLARE
    v_stars INTEGER;
    v_is_demon BOOLEAN;
BEGIN
    IF (TG_OP = 'INSERT') THEN
        SELECT
            COALESCE(stars, 0),
            COALESCE(is_demon, FALSE)
        INTO
            v_stars,
            v_is_demon 
        FROM rates
        WHERE level_id = NEW.level_id;

        UPDATE users 
        SET
            stars = stars + v_stars,
            demons = CASE WHEN v_is_demon THEN demons + 1 ELSE demons END
        WHERE id = NEW.user_id;

        RETURN NEW;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_stats
AFTER INSERT OR UPDATE OR DELETE ON completions
FOR EACH ROW
EXECUTE FUNCTION update_stats();

CREATE MATERIALIZED VIEW user_leaderboard AS
SELECT
    id,
    ROW_NUMBER() OVER (ORDER BY stars DESC, id ASC) AS star_rank,
    ROW_NUMBER() OVER (ORDER BY creator_points DESC, id ASC) AS creator_rank,
    ROW_NUMBER() OVER (ORDER BY demons DESC, id ASC) AS demon_rank,
    ROW_NUMBER() OVER (ORDER BY user_coins DESC, id ASC) AS user_coin_rank,
    ROW_NUMBER() OVER (ORDER BY moons DESC, id ASC) AS moon_rank
FROM users;

CREATE UNIQUE INDEX index_user_leaderboard_id ON user_leaderboard(id);

CREATE VIEW user_view AS
SELECT 
    users.id,
    users.username,
    users.role,
    users.stars,
    users.demons,
    users.creator_points,
    users.diamonds,
    users.moons,
    users.secret_coins,
    users.user_coins,
    users.cube,
    users.ship,
    users.ball,
    users.ufo,
    users.wave,
    users.robot,
    users.spider,
    users.swing,
    users.jetpack,
    users.glow,
    users.explosion,
    users.icon,
    users.icon_type,
    users.color1,
    users.color2,
    users.color3,
    users.accept_friend_requests,
    users.message_setting,
    users.comment_setting,
    users.youtube,
    users.twitter,
    users.twitch,
    users.discord,
    users.instagram,
    users.tiktok,
    users.created_at,

    user_leaderboard.star_rank,
    user_leaderboard.creator_rank,
    user_leaderboard.demon_rank,
    user_leaderboard.user_coin_rank,
    user_leaderboard.moon_rank
FROM users
LEFT JOIN user_leaderboard ON user_leaderboard.id = users.id;

CREATE VIEW level_view AS
SELECT
    levels.*,
    users.username,
    rates.rating,
    COALESCE(rates.stars, 0) AS stars,
    COALESCE(rates.difficulty, 'NA'::difficulty) AS difficulty,
    rates.demon_difficulty,
    COALESCE(rates.has_verified_coins, false) AS has_verified_coins,
    COALESCE(rates.is_auto, false) AS is_auto,
    COALESCE(rates.is_demon, false) AS is_demon,
    COALESCE(rates.is_featured, false) AS is_featured,
    (rates.level_id IS NOT NULL) AS is_rated,
    rates.rated_by,
    rates.rated_at
FROM levels
JOIN users ON levels.user_id = users.id
LEFT JOIN rates ON levels.id = rates.level_id;

CREATE VIEW message_view AS
SELECT
    messages.*,
    users.username
FROM messages
JOIN users ON messages.user_id = users.id;

CREATE VIEW friend_request_view AS
SELECT
    friend_requests.id,
    friend_requests.user_id,
    friend_requests.target_id,
    friend_requests.body,
    friend_requests.is_new,
    friend_requests.created_at,
    users.username,
    users.color1,
    users.color2,
    users.color3,
    users.icon,
    users.icon_type,
    users.glow
FROM friend_requests
JOIN users ON users.id = friend_requests.user_id;

CREATE VIEW comment_view AS
SELECT
    comments.id,
    comments.level_id,
    comments.user_id,
    comments.body,
    comments.likes,
    comments.is_spam,
    comments.created_at,
    comments.percent,
    comments.chat_color,
    users.username,
    users.role,
    users.color1,
    users.color2,
    users.color3,
    users.icon,
    users.icon_type,
    users.glow
FROM comments
JOIN users ON users.id = comments.user_id;

CREATE VIEW list_view AS
SELECT
    lists.*,
    users.username
FROM lists
JOIN users ON lists.user_id = users.id;
