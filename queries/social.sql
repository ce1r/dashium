--! block_user
INSERT INTO blocks (
    user_id,
    target_id
)
SELECT
    :user_id,
    :target_id
WHERE EXISTS (
    SELECT 1
    FROM users
    WHERE id = :user_id
);

--! unblock_user
DELETE FROM blocks
WHERE user_id = :user_id
    AND target_id = :target_id;

--! create_message
INSERT INTO messages (
    user_id,
    target_id,
    subject,
    body
)
SELECT
    :user_id,
    :target_id,
    :subject,
    :body
FROM users
WHERE id = :user_id;
