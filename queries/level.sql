--: Level (featured_at?)

--! create_level
INSERT INTO levels (
    name,
    description,
    user_id,

    version,
    original_level_id,

    length,
    objects,
    requested_stars,
    coins,

    is_auto,
    is_ldm,
    is_two_player,
    is_platformer,

    official_song_id,
    song_id,

    visibility
)
SELECT
    :name,
    :description,
    users.id,

    :version,
    :original_level_id,

    :length,
    :objects,
    :requested_stars,
    :coins,

    :is_auto,
    :is_ldm,
    :is_two_player,
    :is_platformer,

    :official_song_id,
    :song_id,

    :visibility
FROM users
WHERE users.id = :user_id
RETURNING id;

--! search_levels : Level
SELECT
    levels.*,
    users.username
FROM levels
JOIN users ON levels.user_id = users.id
WHERE levels.name ILIKE '%' || :search || '%'
LIMIT 10 OFFSET :offset;

--! get_level : Level
SELECT
    levels.*,
    users.username
FROM levels
JOIN users ON levels.user_id = users.id
WHERE levels.id = :level_id;
