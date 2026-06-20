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
        AND gjp2 = :gjp2
)
AND NOT EXISTS (
    SELECT 1
    FROM blocks
    WHERE user_id = :user_id
        AND target_id = :target_id
);
