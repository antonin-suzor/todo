use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{Data, ThinData, get};
use actix_web::{App, HttpServer};
use sqlx::PgPool;

mod auth;
mod misc;
mod todos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    let app_state: Data<misc::AppStateCounter> = Data::new(misc::AppStateCounter::new());
    let db_pool = PgPool::connect("postgres://root_usr:root_pwd@localhost:5432/root_db")
        .await
        .expect("Connection to database should not fail");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .app_data(ThinData(db_pool.clone()))
            .route("/api", get().to(async || "This is the Rust Actix backend."))
            .service(auth::fake_auth)
            .service(misc::get_count)
            .service(misc::get_count_ghost)
            .service(misc::post_echo)
            .service(misc::get_users)
            .service(todos::post_api_todos)
            .service(todos::patch_api_todos_id)
            .service(todos::delete_api_todos_id)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
