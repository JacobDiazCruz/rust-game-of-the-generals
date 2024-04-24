create table piece (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rank varchar not null,
    name varchar not null,
    player_id UUID REFERENCES player(id),
    game_id UUID REFERENCES game(id),
    square varchar not null,
    eliminations varchar[] not null
);