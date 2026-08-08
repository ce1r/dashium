CREATE TABLE comments (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_id INTEGER NOT NULL REFERENCES levels (id) ON DELETE CASCADE,
    body VARCHAR(100) NOT NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    is_spam BOOLEAN NOT NULL DEFAULT FALSE,
    percent SMALLINT NOT NULL DEFAULT 0 CHECK (percent BETWEEN 0 AND 100),
    chat_color VARCHAR(11) NOT NULL DEFAULT '255,255,255',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE VIEW comment_view AS
SELECT
    comments.id,
    comments.level_id,
    comments.user_id,
    comments.body,
    comments.likes,
    comments.is_spam,
    comments.created_at,
    comments.percent,
    comments.chat_color,
    users.username,
    users.mod_level,
    users.color1,
    users.color2,
    users.color3,
    users.icon,
    users.icon_type,
    users.glow
FROM comments
JOIN users ON users.id = comments.user_id;
