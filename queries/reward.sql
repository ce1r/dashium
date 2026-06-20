--! get_udid
SELECT udid
FROM users
WHERE id = :user_id
    AND gjp2 = :gjp2;
