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
