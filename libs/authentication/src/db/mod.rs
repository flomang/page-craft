use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;