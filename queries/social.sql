--: Message(id, user_id, target_id, subject, body, is_read, created_at, username)

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

--! get_messages : Message
SELECT
    m.*,
    u.username
FROM messages m
JOIN users u ON m.user_id = u.id
WHERE m.target_id = :target_id
ORDER BY m.created_at DESC
LIMIT 10 OFFSET :offset;

--! get_sent_messages : Message
SELECT
    m.*,
    u.username
FROM messages m
JOIN users u ON m.target_id = u.id
WHERE m.user_id = :user_id
ORDER BY m.created_at DESC
LIMIT 10 OFFSET :offset;

--! download_message: Message
WITH updated AS (
    UPDATE messages
    SET is_read = TRUE
    WHERE id = :message_id AND target_id = :target_id
    RETURNING *
)
SELECT updated.*, u.username
FROM updated
JOIN users u ON updated.user_id = u.id;

--! delete_message
DELETE FROM messages
WHERE id = :message_id
    AND target_id = :user_id;

--! delete_sent_message
DELETE FROM messages
WHERE id = :message_id
    AND user_id = :user_id;

--! delete_messages
DELETE FROM messages
WHERE target_id = :user_id
    AND id = ANY(:message_ids);

--! delete_sent_messages
DELETE FROM messages
WHERE user_id = :user_id
    AND id = ANY(:message_ids);

--! create_friend_request
INSERT INTO friend_requests (
    user_id,
    target_id,
    body
) VALUES (
    :user_id,
    :target_id,
    :body
);