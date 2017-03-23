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
