use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::web::Path;
use actix_web::{HttpRequest, HttpResponse, get};

#[get("/api/fake-auth/as/{id}")]
async fn fake_auth(req: HttpRequest, path_params: Path<i32>) -> HttpResponse {
    let id: i32 = path_params.into_inner();
    let host = req
        .headers()
        .get(header::HOST)
        .expect("Header Host should be available")
        .to_str()
        .expect("Host should be a string");

    let cookie: Cookie = if id <= 0 {
        let mut c = Cookie::build("userId", "").domain(host).path("/").finish();
        c.make_removal();
        c
    } else {
        Cookie::build("userId", id.to_string())
            .domain(host)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish()
    };

    HttpResponse::Ok().cookie(cookie).finish()
}
