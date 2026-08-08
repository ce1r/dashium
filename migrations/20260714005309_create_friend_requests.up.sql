CREATE TABLE friend_requests (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    target_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    body VARCHAR(255) NOT NULL,
    is_new BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_friend_request UNIQUE (user_id, target_id),
    CONSTRAINT no_self_friend_request CHECK (user_id != target_id)
);

CREATE VIEW friend_request_view AS
SELECT
    fr.id,
    fr.user_id,
    fr.target_id,
    fr.body,
    fr.is_new,
    fr.created_at,
    u.username,
    u.color1,
    u.color2,
    u.color3,
    u.icon,
    u.icon_type,
    u.glow
FROM friend_requests fr
JOIN users u ON u.id = fr.user_id;
