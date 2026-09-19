CREATE OR REPLACE FUNCTION update_stats()
RETURNS TRIGGER AS $$
DECLARE
    v_stars INTEGER;
    v_user_coins INTEGER;
    v_has_verified_coins BOOLEAN;
    v_is_platformer BOOLEAN;
    v_is_demon BOOLEAN;
BEGIN
    IF (TG_OP = 'INSERT') THEN
        SELECT
            COALESCE(stars, 0),
            COALESCE(has_verified_coins, FALSE),
            COALESCE(is_demon, FALSE)
        INTO
            v_stars,
            v_has_verified_coins,
            v_is_demon 
        FROM rates
        WHERE level_id = NEW.level_id;

        SELECT
            COALESCE(is_platformer, FALSE),
            COALESCE(coins, 0)
        INTO
            v_is_platformer,
            v_user_coins
        FROM levels
        WHERE id = NEW.level_id;

        UPDATE users 
        SET
            stars = CASE WHEN NOT v_is_platformer THEN stars + v_stars ELSE stars END,
            moons = CASE WHEN v_is_platformer THEN moons + v_stars ELSE moons END,
            demons = CASE WHEN v_is_demon THEN demons + 1 ELSE demons END,
            user_coins = CASE WHEN v_has_verified_coins THEN user_coins + v_user_coins ELSE user_coins END
        WHERE id = NEW.user_id;

        RETURN NEW;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_creator_points()
RETURNS TRIGGER AS $$
DECLARE
    v_user_id INTEGER;
BEGIN
    IF (TG_OP = 'INSERT') THEN
        SELECT user_id
        INTO v_user_id
        FROM levels
        WHERE id = NEW.level_id;

        UPDATE users
        SET creator_points = creator_points + CASE NEW.rating
            WHEN 'Star' THEN 1
            WHEN 'Feature' THEN 1
            WHEN 'Epic' THEN 2
            WHEN 'Legendary' THEN 3
            WHEN 'Mythic' THEN 4
            ELSE 0
        END
        WHERE id = v_user_id;

        RETURN NEW;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
