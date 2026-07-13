--! get_quests
SELECT *
FROM quests
ORDER BY created_at DESC
LIMIT 3;
