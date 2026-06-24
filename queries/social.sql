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
);

--! unblock_user
DELETE FROM blocks
USING users
WHERE blocks.user_id = users.id
    AND blocks.user_id = :user_id
    AND blocks.target_id = :target_id
    AND users.gjp2 = :gjp2;
