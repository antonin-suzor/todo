use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use actix_cors::Cors;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_web::web::{get, scope, Data, Json, ThinData};
use serde::Serialize;
use sqlx::{PgPool, Row};

pub mod todos;

struct AppStateCounter {
    count: RwLock<u64>,
}

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

#[derive(Serialize)]
struct UserData {
    id: i32,
    name: String,
}

#[get("/count")]
async fn get_count(data: Data<AppStateCounter>) -> impl Responder {
    let mut count: RwLockWriteGuard<u64> = data.count.write().expect("AppState should be available");
    *count += 1;
    return Json(CounterResponse { count: *count });
}

#[get("/count/ghost")]
async fn get_count_ghost(data: Data<AppStateCounter>) -> impl Responder {
    let count: RwLockReadGuard<u64> = data.count.read().expect("AppState should be available");
    return Json(CounterResponse { count: *count });
}

#[post("/echo")]
async fn post_echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body)
}

#[get("/users")]
async fn get_users(ThinData(db_pool): ThinData<PgPool>) -> impl Responder {
    let q = sqlx::query("SELECT id, name FROM account;").fetch_all(&db_pool).await;
    return match q {
        Ok(v) => HttpResponse::Ok().json(
            v.iter()
                .map(|row| {
                    let id: i32 = row.get("id");
                    let name: String = row.get("name");
                    return UserData { id, name };
                })
                .collect::<Vec<UserData>>()
        ),
        Err(_) => HttpResponse::InternalServerError().await.expect("HttpResponse should build"),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe { std::env::set_var("RUST_LOG", "debug"); }
    env_logger::init();
    let app_state: Data<AppStateCounter> = Data::new(AppStateCounter {
        count: RwLock::new(0),
    });
    let db_pool= PgPool::connect("postgres://root_usr:root_pwd@localhost:5432/root_db").await.expect("Connection to database should not fail");
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .service(scope("/rust")
            .wrap(cors)
            .app_data(app_state.clone())
            .app_data(ThinData(db_pool.clone()))
            .service(todos::post_api_todos)
            .service(get_count)
            .service(get_count_ghost)
            .service(post_echo)
            .service(get_users)
            .route("", get().to(async || { "This is the Rust Actix backend." })))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
