use actix_web::{post, HttpRequest, HttpResponse};
use actix_web::web::{Json, ThinData};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};


#[derive(Deserialize, Serialize)]
struct TODOElementData {
    id: i32,
    title: String,
    done: bool,
    description: String,
    account_id: Option<i32>,
}
#[derive(Deserialize, Serialize)]
struct TODOElementDataRequest {
    title: String,
    done: bool,
    description: String,
}

#[post("/api/todos")]
pub async fn post_api_todos(req: HttpRequest, todo: Json<TODOElementDataRequest>, ThinData(db_pool): ThinData<PgPool>) -> HttpResponse {
    let user_id_from_cookie: Option<i32> = match req.cookie("userId") {
        Some(cookie) => match cookie.value().parse() {
            Ok(user_id) => Some(user_id),
            Err(_) => None,
        }
        None => None,
    };

    let q = sqlx::query("INSERT INTO todo_element_data (title, done, description, account_id) VALUES ($1, $2, $3, $4) RETURNING id;")
        .bind(todo.title.as_str())
        .bind(todo.done)
        .bind(todo.description.as_str())
        .bind(user_id_from_cookie)
        .fetch_one(&db_pool).await;
    if q.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let q = q.unwrap();

    return HttpResponse::Ok()
        .json(TODOElementData {
            id: q.get("id"),
            title: todo.title.clone(),
            done: todo.done,
            description: todo.description.clone(),
            account_id: user_id_from_cookie,
        });
}