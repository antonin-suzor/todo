use std::sync::{Mutex, MutexGuard};
use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

struct AppState {
    count: Mutex<u64>,
}

#[derive(Serialize)]
struct CounterResponse {
    count: u64,
}

#[get("/count")]
async fn count(data: web::Data<AppState>) -> impl Responder {
    let mut count: MutexGuard<u64> = data.count.lock().expect("AppState should be available");
    *count += 1;
    return web::Json(CounterResponse { count: *count });
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        count: Mutex::new(0),
    });
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(count)
            .service(echo)
            .route("/", web::get().to(async || { "This is the Rust Actix backend." }))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
