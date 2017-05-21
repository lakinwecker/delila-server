CREATE TABLE database (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  date_created VARCHAR NOT NULL,
  date_modified VARCHAR NOT NULL
);
CREATE TABLE tag (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL
);
CREATE TABLE database_tag (
  id INTEGER PRIMARY KEY NOT NULL,
  database_id INTEGER NOT NULL,
  tag_id INTEGER NOT NULL
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
    pgn VARCHAR NOT NULL
);
