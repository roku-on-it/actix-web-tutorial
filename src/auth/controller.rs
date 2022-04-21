use actix_web::{post, web, HttpResponse, Responder};
use diesel::NotFound;

use crate::auth::model::LoginInput;
use crate::user::model::User;
use crate::AppState;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/auth").service(login));
}

#[post("/login")]
pub async fn login(app_state: web::Data<AppState>, input: web::Json<LoginInput>) -> impl Responder {
  let user = User::find_one_by_email(input.0.email, &app_state.db_pool);
  match user {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(why) => match why {
      NotFound => HttpResponse::NotFound().body("Not Found Exception"),
      _ => HttpResponse::InternalServerError().body("Internal server error"),
    },
  }
}
