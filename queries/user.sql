--! get_hash_and_salt
SELECT
    hash,
    salt
FROM users
WHERE id = :user_id;

--! create_user
INSERT INTO users (
    username,
    email,
    hash,
    salt
) VALUES (
    :username,
    :email,
    :hash,
    :salt
);

--! get_user
SELECT
    id,
    username,
    is_activated,
    mod_level,

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

    message_setting,
    friend_setting,
    comment_setting,

    youtube,
    twitter,
    twitch,
    discord,
    instagram,
    tiktok
FROM users
WHERE id = :id;

--! login_user
UPDATE users
SET udid = :udid
WHERE username = :username
RETURNING id, hash, salt;

--! save_data
UPDATE users
SET save_data = :save_data
WHERE id = :user_id;

--! load_data
SELECT save_data
FROM users
WHERE id = :user_id;

--! save_stats
UPDATE users
SET
    stars = :stars,
    demons = :demons,
    diamonds = :diamonds,
    moons = :moons,
    secret_coins = :secret_coins,
    user_coins = :user_coins,

    cube = :cube,
    ship = :ship,
    ball = :ball,
    ufo = :ufo,
    wave = :wave,
    robot = :robot,
    spider = :spider,
    swing = :swing,
    jetpack = :jetpack,
    glow = :glow,
    explosion = :explosion,
    icon = :icon,
    icon_type = :icon_type,

    color1 = :color1,
    color2 = :color2,
    color3 = :color3
WHERE id = :user_id
RETURNING id;

--! get_mod_level
SELECT mod_level
FROM users
WHERE id = :user_id;

--! update_settings
UPDATE users
SET
    message_setting = :message_setting,
    friend_setting = :friend_setting,
    comment_setting = :comment_setting,
    youtube = :youtube,
    twitter = :twitter,
    twitch = :twitch,
    discord = :discord,
    instagram = :instagram,
    tiktok = :tiktok
WHERE id = :user_id;

--! search_users
SELECT
    id,
    username,

    stars,
    demons,
    creator_points,
    diamonds,
    moons,
    secret_coins,
    user_coins,

    glow,
    icon,
    icon_type,

    color1,
    color2,
    color3
FROM users
WHERE username ILIKE '%' || :search || '%'
    AND id != :user_id
LIMIT 10 OFFSET :offset;

--! get_udid
SELECT udid
FROM users
WHERE id = :user_id;
