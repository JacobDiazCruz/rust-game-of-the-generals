create table piece (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name varchar not null,
    player_id UUID REFERENCES player(id),
    game_id UUID REFERENCES game(id),
    square varchar not null
);