use actix_web::{get, web, HttpResponse, Responder};
use crate::user::service::UserService;
use crate::state::AppState;

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_users);
    cfg.service(get_user);
}

#[get("/user")]
async fn list_users(data: web::Data<AppState>) -> impl Responder {
    let user_service = UserService::new(data.connection.clone());
    match user_service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_error) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/user/{id}")]
async fn get_user(id: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let user_service = UserService::new(data.connection.clone());
    let user_id = id.into_inner();
    match user_service.get_user_info(user_id).await {
        Ok(user) => match user {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_error) => HttpResponse::InternalServerError().finish(),
    }
}

