use chrono::prelude::*;

#[derive(Queryable,Serialize,Deserialize)]
pub struct Database {
    pub id: i32,
    pub title: String,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Tag {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct DatabaseTag {
    pub id: i32,
    pub tag_id: i32,
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
