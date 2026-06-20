--! create_post
INSERT INTO posts (
    user_id,
    body
)
SELECT
    :user_id,
    :body
FROM users
WHERE id = :user_id
    AND gjp2 = :gjp2
RETURNING id;
