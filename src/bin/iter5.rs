use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;

#[path = "../iter5/errors.rs"]
mod errors;

#[path = "../iter5/handlers/mod.rs"]
mod handlers;

#[path = "../iter5/models/mod.rs"]
mod models;

#[path = "../iter5/routes.rs"]
mod routes;

#[path = "../iter5/state.rs"]
mod state;

use routes::*;
use state::AppState;
use crate::errors::EzyTutorError;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorError::InvalidInput(
                    "Please provide valid json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };

    let host_port = env::var("HOST_PORT").expect("HOST_PORT must be set");
    HttpServer::new(app).bind(&host_port)?.run().await
}