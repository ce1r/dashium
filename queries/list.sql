--! create_list
INSERT INTO lists (
    name,
    description,
    user_id,
    levels,
    difficulty
)
SELECT
    :name,
    :description,
    :user_id,
    :levels,
    :difficulty
FROM users
WHERE id = :user_id
RETURNING id;
