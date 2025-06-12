use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use actix_web::{get, post, HttpResponse, Responder};
use actix_web::web::{Data, Json, ThinData};
use serde::Serialize;
use sqlx::{PgPool, Row};

pub struct AppStateCounter {
    count: RwLock<u64>,
}
impl AppStateCounter {
    pub fn new() -> AppStateCounter {
        Self {
            count: RwLock::new(0),
        }
    }
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

#[get("/api/count")]
async fn get_count(data: Data<AppStateCounter>) -> impl Responder {
    let mut count: RwLockWriteGuard<u64> = data.count.write().expect("AppState should be available");
    *count += 1;
    return Json(CounterResponse { count: *count });
}

#[get("/api/count/ghost")]
async fn get_count_ghost(data: Data<AppStateCounter>) -> impl Responder {
    let count: RwLockReadGuard<u64> = data.count.read().expect("AppState should be available");
    return Json(CounterResponse { count: *count });
}

#[post("/api/echo")]
async fn post_echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body)
}

#[get("/api/users")]
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
