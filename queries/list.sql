--: List() : serde::Serialize

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

--! search_lists : List
SELECT *
FROM list_view
WHERE name ILIKE '%' || :search || '%'
LIMIT 10 OFFSET :offset;

--! delete_list
DELETE FROM lists
WHERE id = :list_id
    AND user_id = :user_id;
