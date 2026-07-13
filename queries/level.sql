--! create_level
INSERT INTO levels (
    name,
    description,
    user_id,
    level_data,

    version,
    original_level_id,

    length,
    objects,
    requested_stars,
    coins,

    is_auto,
    is_ldm,
    is_two_player,

    official_song_id,
    song_id,

    visibility
)
SELECT
    :name,
    :description,
    users.id,
    :level_data,

    :version,
    :original_level_id,

    :length,
    :objects,
    :requested_stars,
    :coins,

    :is_auto,
    :is_ldm,
    :is_two_player,

    :official_song_id,
    :song_id,

    :visibility
FROM users
WHERE id = :user_id
RETURNING id;
