--: User() : serde::Serialize

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

--! login_user
UPDATE users
SET udid = :udid
WHERE username = :username
RETURNING id, hash, salt;

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
    accept_friend_requests = :accept_friend_requests,
    message_setting = :message_setting,
    comment_setting = :comment_setting,
    youtube = :youtube,
    twitter = :twitter,
    twitch = :twitch,
    discord = :discord,
    instagram = :instagram,
    tiktok = :tiktok
WHERE id = :user_id;

--! get_udid
SELECT udid
FROM users
WHERE id = :user_id;

--! get_user_by_username : User
SELECT *
FROM user_view
WHERE username = :username;

--! get_user_by_id : User
SELECT *
FROM user_view
WHERE id = :id;

--! search_users : User
SELECT *
FROM user_view
WHERE username ILIKE '%' || :search || '%'
    AND id != :user_id
LIMIT 10 OFFSET :offset;
