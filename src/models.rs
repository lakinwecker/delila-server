#[derive(Queryable,Serialize,Deserialize)]
pub struct Database {
    pub id: i32,
    pub title: String,
    pub description: String
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct Tag {
    pub id: i32,
    pub title: String
}

#[derive(Queryable,Serialize,Deserialize)]
pub struct DatabaseTag {
    pub id: i32,
    pub title: String
}
