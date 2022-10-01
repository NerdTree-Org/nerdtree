use super::super::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct PostModel {
    pub id: uuid::Uuid,
    pub is_approved: bool,
    pub title: String,
    pub thumbnail: Option<String>,
    pub body: String,
    pub creation_date: chrono::NaiveDateTime,
    pub approval_date: Option<chrono::NaiveDateTime>,
    pub post_author: Option<uuid::Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub post_author: &'a uuid::Uuid,
}
