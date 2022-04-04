use actix_web::{get, post, web};
use actix_web::{HttpResponse, Responder};

use crate::user::model::{CreateUser, User};
use crate::AppState;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(get_users).service(create_user));
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
