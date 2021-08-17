use actix_web::web::{Json, Data};
use crate::services::blog::payload::{PaginatePostsPayload, PaginatePostsReturnPayload, GetPostByIdPayload, GetPostsByAuthorId};
use crate::db::Pool;
use actix_web::Responder;
use crate::errors::Errors;
use crate::db::post::query::{get_all_posts, get_post_by_uuid, get_posts_by_author_id};
use actix_web_validator::Json as Validate;
use uuid::Uuid;
use std::str::FromStr;

struct PaginatedVec<'a, T: 'a> {
    per_page: usize,
    pages: Vec<Vec<&'a T>>
}

impl<'a, T> PaginatedVec<'a, T> {
    pub fn from_vec(vec: &'a Vec<T>, per_page: usize) -> PaginatedVec<T> {
        PaginatedVec {
            per_page,
            pages: vec.chunks(per_page).map(|page| {
                page.iter().collect::<Vec<&'a T>>()
            }).collect::<Vec<Vec<&'a T>>>()
        }
    }

    pub fn page(&'a self, index: usize) -> Option<(usize, &Vec<&'a T>)> {
        self.pages.get(index).map(|page| {
            (
                index % self.per_page,
                page
            )
        })
    }

    pub fn get_max_pages(&self) -> usize {
        self.pages.len()
    }
}

pub async fn paginate_posts_handler(
    payload: Validate<PaginatePostsPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let posts = get_all_posts(&conn_pool)?;
    let pg_vec = PaginatedVec::from_vec(&posts, payload.per_page);
    let page = pg_vec.page(payload.page - 1);

    // I don't know what the fuck I'm doing. Probably a piece of code that'll make this goddamn thing slower
    return if page.is_none() {
        Ok(Json(PaginatePostsReturnPayload {
            current_page: payload.page,
            max_page: pg_vec.get_max_pages(),
            page: Vec::new()
        }))
    } else {
        Ok(Json(PaginatePostsReturnPayload {
            current_page: page.unwrap().0 + 1,
            max_page: pg_vec.get_max_pages(),
            page: page.unwrap().1.into_iter().map(|v| v.clone().clone()).collect()
        }))
    }
}

pub async fn get_post_by_id_handler(
    payload: Json<GetPostByIdPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    Ok(Json(get_post_by_uuid(Uuid::from_str(&payload.post_id)
                             .map_err(|e| Errors::BadRequest(e.to_string()))?, &conn_pool)?
            .get(0)
            .ok_or_else(|| Errors::BadRequest(String::from("No such post!")))?
            .clone()))
}

pub async fn get_posts_by_author_id_handler(
    payload: Json<GetPostsByAuthorId>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let posts = get_posts_by_author_id(Uuid::from_str(&payload.author_id)
                                           .map_err(|e| Errors::BadRequest(e.to_string()))?, &conn_pool)?;
    Ok(Json(posts))
}