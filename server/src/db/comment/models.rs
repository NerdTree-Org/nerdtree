use crate::db::schema::comments;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Clone, Debug, Serialize, Deserialize)]
pub struct CommentModel {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Option<Uuid>,
    pub body: String,
    pub creation_date: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub post_id: &'a Uuid,
    pub author_id: &'a Uuid,
    pub body: &'a str,
}
