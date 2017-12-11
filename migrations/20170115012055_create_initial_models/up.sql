CREATE TABLE position (
    id INTEGER PRIMARY KEY NOT NULL,
    hash_1 INTEGER NOT NULL,
    hash_2 INTEGER NOT NULL
);
CREATE TABLE _move (
    id INTEGER PRIMARY KEY NOT NULL,
    uci INTEGER not null,
    starting_position_id INTEGER NOT NULL,
    ending_position_id INTEGER NOT NULL
);
CREATE TABLE line_move (
    id INTEGER PRIMARY KEY NOT NULL,
    move_id INTEGER NOT NULL,
    line_id INTEGER NOT NULL,
    -- The ply implied colour. n % 2 == 0 -> black || n % 2 == 1 -> white
    ply INTEGER NOT NULL
);
CREATE TABLE line (
    id INTEGER PRIMARY KEY NOT NULL,
    starting_position_id INTEGER NOT NULL,
    parent_line_id INTEGER NULL
);
CREATE TABLE player (
    id INTEGER PRIMARY KEY NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    middle_name VARCHAR NULL
);
CREATE TABLE event (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    year INTEGER NOT NULL
);
CREATE TABLE site (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL
);
CREATE TABLE game (
    id INTEGER PRIMARY KEY NOT NULL,
    white_player_id INTEGER NOT NULL,
    white_player_rating INTEGER NOT NULL,
    black_player_id INTEGER NOT NULL,
    black_player_rating INTEGER NOT NULL,
    event_id INTEGER NULL,
    site_id INTEGER NULL,
    date VARCHAR NOT NULL,
    round INTEGER NULL,
    result VARCHAR NOT NULL,
    pgn VARCHAR NOT NULL,
    -- the line that represents the deconstructed game.
    line_id INTEGER NOT NULL 
);
