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
    c.id,
    c.level_id,
    c.user_id,
    c.body,
    c.likes,
    c.is_spam,
    c.created_at,
    c.percent,
    c.chat_color,
    u.username,
    u.role,
    u.color1,
    u.color2,
    u.color3,
    u.icon,
    u.icon_type,
    u.glow
FROM comments c
JOIN users u ON u.id = c.user_id;
