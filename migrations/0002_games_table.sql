create table game (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    white_player_id UUID NOT NULL,
    black_player_id UUID NOT NULL,
    turn_player_id UUID NOT NULL,
    winner_player_id UUID,
    FOREIGN KEY (white_player_id) REFERENCES player(id),
    FOREIGN KEY (black_player_id) REFERENCES player(id),
    FOREIGN KEY (turn_player_id) REFERENCES player(id),
    FOREIGN KEY (winner_player_id) REFERENCES player(id)
);