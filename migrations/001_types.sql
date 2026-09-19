CREATE TYPE role AS ENUM (
    'User',
    'Moderator',
    'ElderModerator',
    'LeaderboardModerator',
    'Administrator'
);

CREATE TYPE message_setting AS ENUM (
    'All',
    'FriendsOnly',
    'None'
);

CREATE TYPE comment_setting AS ENUM (
    'All',
    'FriendsOnly',
    'None'
);

CREATE TYPE level_length AS ENUM (
    'Tiny',
    'Short',
    'Medium',
    'Long',
    'XL'
);

CREATE TYPE visibility AS ENUM (
    'Public',
    'FriendsOnly',
    'Private'
);

CREATE TYPE rating AS ENUM (
    'Star',
    'Feature',
    'Epic',
    'Legendary',
    'Mythic'
);

CREATE TYPE demon_difficulty AS ENUM (
    'Easy',
    'Medium',
    'Hard',
    'Insane',
    'Extreme'
);

CREATE TYPE difficulty AS ENUM (
    'NA',
    'Auto',
    'Easy',
    'Normal',
    'Hard',
    'Harder',
    'Insane',
    'Demon'
);

CREATE TYPE item_type AS ENUM (
    'Orbs',
    'Coins',
    'Stars'
);
