use super::super::schema::users;

#[derive(Queryable, Clone)]
pub struct UserModel {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub profile_pic: Option<String>,
    pub is_admin: bool,
    pub facebook_id: String,
    pub discord_token: uuid::Uuid,
    pub is_discord_token_used: bool,
    pub joined_date: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub facebook_id: &'a str,
}
