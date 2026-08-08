--: Level (demon_difficulty?, rated_by?, rated_at?, rating?) : serde::Serialize

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
SELECT *
FROM level_view
WHERE name ILIKE '%' || :search || '%'
LIMIT 10 OFFSET :offset;

--! get_level : Level
SELECT *
FROM level_view
WHERE id = :level_id;

--! get_levels_of_user : Level
SELECT level_view.*
FROM level_view
WHERE user_id = :user_id;

--! get_level_count
SELECT COUNT(*)
FROM levels;

--! delete_level
DELETE FROM levels
WHERE id = :level_id
    AND user_id = :user_id;
