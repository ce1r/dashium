DROP TYPE IF EXISTS role;
CREATE TYPE role AS ENUM (
    'User',
    'Moderator',
    'ElderModerator',
    'LeaderboardModerator',
    'Administrator'
);

DROP TYPE IF EXISTS message_setting;
CREATE TYPE message_setting AS ENUM (
    'All',
    'FriendsOnly',
    'None'
);

DROP TYPE IF EXISTS comment_setting;
CREATE TYPE comment_setting AS ENUM (
    'All',
    'FriendsOnly',
    'None'
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

CREATE VIEW user_view AS
SELECT 
    id,
    username,
    role,
    stars,
    demons,
    creator_points,
    diamonds,
    moons,
    secret_coins,
    user_coins,
    cube,
    ship,
    ball,
    ufo,
    wave,
    robot,
    spider,
    swing,
    jetpack,
    glow,
    explosion,
    icon,
    icon_type,
    color1,
    color2,
    color3,
    accept_friend_requests,
    message_setting,
    comment_setting,
    youtube,
    twitter,
    twitch,
    discord,
    instagram,
    tiktok,
    created_at,
    ROW_NUMBER() OVER (ORDER BY stars DESC, id ASC) AS star_rank,
    ROW_NUMBER() OVER (ORDER BY creator_points DESC, id ASC) AS creator_rank,
    ROW_NUMBER() OVER (ORDER BY demons DESC, id ASC) AS demon_rank,
    ROW_NUMBER() OVER (ORDER BY user_coins DESC, id ASC) AS user_coin_rank,
    ROW_NUMBER() OVER (ORDER BY moons DESC, id ASC) AS moon_rank
FROM users;
