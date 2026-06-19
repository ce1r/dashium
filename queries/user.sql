--: User(id, username, email, created_at)

--! create_user
INSERT INTO users (
    username,
    email,
    gjp2
) VALUES (
    :username,
    :email,
    :gjp2
);

--! get_user_by_username : User
SELECT
    id,
    username,
    email,
    created_at
FROM users
WHERE username = :username;

--! is_username_taken
SELECT EXISTS (
    SELECT 1
    FROM users
    WHERE username ILIKE :username
);

--! is_email_taken
SELECT EXISTS (
    SELECT 1
    FROM users
    WHERE email ILIKE :email
);
