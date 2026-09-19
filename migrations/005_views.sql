CREATE MATERIALIZED VIEW user_leaderboard AS
SELECT
    id,
    ROW_NUMBER() OVER (ORDER BY stars DESC, id ASC) AS star_rank,
    ROW_NUMBER() OVER (ORDER BY creator_points DESC, id ASC) AS creator_rank,
    ROW_NUMBER() OVER (ORDER BY demons DESC, id ASC) AS demon_rank,
    ROW_NUMBER() OVER (ORDER BY user_coins DESC, id ASC) AS user_coin_rank,
    ROW_NUMBER() OVER (ORDER BY moons DESC, id ASC) AS moon_rank
FROM users;

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
