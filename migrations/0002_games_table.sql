create table game (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    white_player_id UUID REFERENCES player(id),
    black_player_id UUID REFERENCES player(id),
    turn_player_id UUID REFERENCES player(id),
    winner_player_id UUID REFERENCES player(id)
);