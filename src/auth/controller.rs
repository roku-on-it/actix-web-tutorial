use actix_web::{HttpResponse, Responder, web};
use actix_web::post;
use diesel::NotFound;

use crate::AppState;
use crate::auth::model::LoginInput;
use crate::core::exception::NotFoundException;
use crate::user::model::User;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/auth").service(login));
}

#[post("/login")]
pub async fn login(app_state: web::Data<AppState>, input: web::Json<LoginInput>) -> impl Responder {
  let user = User::find_one_by_email(input.0.email, &app_state.db_pool);
  match user {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(why) => match why {
      NotFound => HttpResponse::NotFound().json(NotFoundException::default()),
      _ => HttpResponse::InternalServerError().json("Internal server error"),
    },
  }
}
