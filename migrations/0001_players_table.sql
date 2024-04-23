create table player (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_name varchar not null,
    player_color varchar not null
);