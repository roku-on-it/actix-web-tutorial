#[macro_use]
extern crate diesel;

use actix_web::{middleware, web::Data, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod auth;
mod core;
mod post;
mod schema;
mod user;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct AppState {
  db_pool: Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=debug");

  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  let pool: Pool = r2d2::Pool::builder()
    .max_size(4)
    .build(manager)
    .expect("Failed to create pool");

  HttpServer::new(move || {
    App::new()
      .app_data(Data::new(AppState {
        db_pool: pool.clone(),
      }))
      .wrap(middleware::Compress::default())
      .configure(auth::controller::config_routes)
      .configure(user::controller::config_routes)
      .configure(post::controller::config_routes)
  })
  .bind(("127.0.0.1", 4000))?
  .run()
  .await
}
