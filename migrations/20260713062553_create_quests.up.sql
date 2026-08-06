DROP TYPE IF EXISTS item_type;
CREATE TYPE item_type AS ENUM (
    'Orbs',
    'Coins',
    'Stars'
);

CREATE TABLE quests (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    item_type item_type NOT NULL,
    amount SMALLINT NOT NULL CHECK (amount > 0),
    reward SMALLINT NOT NULL CHECK (reward > 0),
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO quests (item_type, amount, reward, name)
VALUES ('Orbs', 2000, 30, 'Gather 2000 Orbs'),
    ('Coins', 5, 20, 'Earn 5 Coins'),
    ('Stars', 10, 10, 'Collect 10 Stars');
