use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::core::exception::Exception;
use crate::post::model::Post;
use crate::schema::posts::dsl::*;
use crate::user::model::{CreateUser, User};
use crate::AppState;

#[derive(Serialize, Deserialize)]
struct UserPost {
  id: i32,
  email: String,
  username: String,
  created_at: chrono::NaiveDateTime,
  updated_at: chrono::NaiveDateTime,
  deleted_at: Option<chrono::NaiveDateTime>,
  posts: Vec<Post>,
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/users")
      .service(get_users)
      .service(create_user)
      .service(get_user),
  );
}

#[get("/{id}")]
pub async fn get_user(app_state: web::Data<AppState>, path: Path<i32>) -> impl Responder {
  let query_id = path.into_inner();
  let user = User::find_one(query_id, &app_state.db_pool);
  let postsa: Vec<Post> = posts
    .filter(user_id.eq(&query_id))
    .load(&app_state.db_pool.get().unwrap())
    .unwrap();

  match user {
    Ok(user) => HttpResponse::Ok().json(UserPost {
      id: user.id,
      email: user.email,
      username: user.username,
      created_at: user.created_at,
      updated_at: user.updated_at,
      deleted_at: user.deleted_at,
      posts: postsa,
    }),
    Err(why) => match why {
      Exception::NotFound => HttpResponse::NotFound().body("Not found"),
      _ => HttpResponse::InternalServerError().body("Internal server error"),
    },
  }
}

#[get("")]
pub async fn get_users(app_state: web::Data<AppState>) -> impl Responder {
  let users = User::find(&app_state.db_pool).unwrap();
  HttpResponse::Ok().json(users)
}

#[post("")]
pub async fn create_user(
  app_state: web::Data<AppState>,
  input: web::Json<CreateUser>,
) -> impl Responder {
  let user = User::save(input.0, &app_state.db_pool).unwrap();
  HttpResponse::Ok().json(user)
}
