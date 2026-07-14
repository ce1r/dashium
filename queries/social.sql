--: Message()
--: FriendRequest()
--: User ()

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

--! get_friend_requests: FriendRequest
SELECT
    fr.id,
    fr.user_id,
    fr.target_id,
    fr.body,
    fr.is_new,
    fr.created_at,
    u.username,
    u.icon,
    u.color1,
    u.color2,
    u.icon_type,
    u.glow
FROM friend_requests fr
JOIN users u on u.id = fr.user_id
WHERE fr.target_id = :user_id
ORDER BY fr.created_at DESC
LIMIT 20 OFFSET :offset;

--! get_sent_friend_requests: FriendRequest
SELECT
    fr.id,
    fr.user_id,
    fr.target_id,
    fr.body,
    fr.is_new,
    fr.created_at,
    u.username,
    u.icon,
    u.color1,
    u.color2,
    u.icon_type,
    u.glow
FROM friend_requests fr
JOIN users u on u.id = fr.target_id
WHERE fr.user_id = :user_id
ORDER BY fr.created_at DESC
LIMIT 20 OFFSET :offset;

--! read_friend_request
UPDATE friend_requests
SET is_new = FALSE
WHERE id = :request_id;

--! accept_friend_request
WITH _ AS (
    DELETE FROM friend_requests
    WHERE user_id = :user_id
        AND target_id = :target_id
)
INSERT INTO friendships (
    user1,
    user2
) VALUES (
    :user_id,
    :target_id
);

--! get_friend_list: User
SELECT u.*
FROM users u
WHERE u.id IN (
    SELECT user2 FROM friendships WHERE user1 = :user_id
    UNION
    SELECT user1 FROM friendships WHERE user2 = :user_id
)
ORDER BY u.username ASC;

--! get_blocked_list: User
SELECT u.*
FROM users u
WHERE u.id IN (
    SELECT target_id
    FROM blocks
    WHERE user_id = :user_id
)
ORDER BY u.username ASC;
