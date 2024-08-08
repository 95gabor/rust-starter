mod migration;
mod state;
mod user;
mod main_controller;

use actix_web::{web, App, HttpServer};
use migration::{Migrator,MigratorTrait};
use sea_orm::{Database};
use state::AppState;
use user::controller::init_user_routes;
use main_controller::hello_world;
use dotenv::dotenv;
use std::env;
use log::info;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "7878".to_string())
        .parse()
        .expect("PORT must be a valid u16 number");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = Database::connect(&database_url).await.unwrap();
    Migrator::up(&connection, None).await.unwrap();
    let state = AppState { connection };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(hello_world))
            .configure(init_user_routes)
    })
    .bind(("0.0.0.0", port))?;

    info!("Starting application on port: {}", port);

    server.run().await
}
