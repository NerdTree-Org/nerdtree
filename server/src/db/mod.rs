use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use r2d2::Error;

pub mod comment;
pub mod post;
pub mod schema;
pub mod user;
pub mod voting;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to make connection")
}

pub fn get_conn(pool: &Pool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    pool.get()
}
