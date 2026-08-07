--: Comment() : serde::Serialize

--! create_comment
INSERT INTO comments (
    user_id,
    level_id,
    comment,
    percent
) VALUES (
    :user_id,
    :level_id,
    :comment,
    :percent
)
RETURNING id;

--! get_comments_by_date : Comment
SELECT *
FROM comment_view
WHERE level_id = :level_id
ORDER BY created_at DESC
LIMIT 20 OFFSET :offset;

--! get_comments_by_likes : Comment
SELECT *
FROM comment_view
WHERE level_id = :level_id
ORDER BY likes DESC
LIMIT 20 OFFSET :offset;


