--! create_user
INSERT INTO users (
    username,
    email,
    gjp2
) VALUES (
    :username,
    :email,
    :gjp2
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

--! is_username_taken
SELECT EXISTS (
    SELECT 1
    FROM users
    WHERE username ILIKE :username
);

--! is_email_taken
SELECT EXISTS (
    SELECT 1
    FROM users
    WHERE email ILIKE :email
);

--! verify_gjp2
SELECT id
FROM users
WHERE username = :username
    AND gjp2 = :gjp2;

--! save_data
UPDATE users
SET save_data = :save_data
WHERE id = :user_id
    AND gjp2 = :gjp2;

--! load_data
SELECT save_data
FROM users
WHERE id = :user_id
    AND gjp2 = :gjp2;
