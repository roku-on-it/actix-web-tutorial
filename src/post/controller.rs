use actix_web::web::Path;
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::core::exception::Exception;
use crate::post::model::{CreatePost, Post};
use crate::AppState;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/posts")
      .service(get_posts)
      .service(create_post)
      .service(get_post),
  );
}

#[get("/{id}")]
pub async fn get_post(app_state: web::Data<AppState>, path: Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let post = Post::find_one(id, &app_state.db_pool);

  match post {
    Ok(post) => HttpResponse::Ok().json(post),
    Err(why) => match why {
      Exception::NotFound => HttpResponse::NotFound().body("Not found"),
      _ => HttpResponse::InternalServerError().body("Internal server error"),
    },
  }
}

#[get("")]
pub async fn get_posts(app_state: web::Data<AppState>) -> impl Responder {
  let posts = Post::find(&app_state.db_pool).unwrap();
  HttpResponse::Ok().json(posts)
}

#[post("")]
pub async fn create_post(
  app_state: web::Data<AppState>,
  input: web::Json<CreatePost>,
) -> impl Responder {
  let post = Post::save(input.0, &app_state.db_pool);
  match post {
    Ok(post) => HttpResponse::Ok().json(post),
    Err(exception) => match exception {
      Exception::NotFound => HttpResponse::NotFound().body("User not found"),
      _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    },
  }
}
