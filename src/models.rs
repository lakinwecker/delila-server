use chrono::prelude::*;

#[derive(Queryable,Serialize,Deserialize)]
pub struct Position {
    pub id: i64,
    pub hash_1: i64,
    pub hash_2: i64,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct _Move {
    pub id: i64,
    pub uci: String,
    pub starting_position_id: i64,
    pub ending_position_id: i64,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct LineMove {
    pub id: i64,
    pub move_id: i64,
    pub tag_id: i64,
    pub line_id: i64,
    pub ply: i16,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middlename: Option<String>,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub city: String,
    pub country: String,
    pub year: i16
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Site {
    pub id: i32,
    pub name: String
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Game {
    pub id: i32,
    pub white_player_id: i32,
    pub white_player_rating: i16,
    pub black_player_id: i32,
    pub black_player_rating: i16,
    pub event_id: i32,
    pub site_id: i32,
    pub date: NaiveDate,
    pub round: i16,
    pub result: String
}
