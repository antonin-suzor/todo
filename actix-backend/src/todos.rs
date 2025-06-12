use actix_web::{delete, patch, post, HttpRequest, HttpResponse};
use actix_web::web::{Json, Path, ThinData};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};


#[derive(Deserialize, FromRow, Serialize)]
struct TODOElementData {
    id: i32,
    title: String,
    done: bool,
    description: String,
    account_id: Option<i32>,
}
impl TODOElementData {
    fn new(id: i32, ed_request: TODOElementDataRequest, account_id: Option<i32>) -> Self {
        Self {
            id,
            title: ed_request.title.unwrap_or(String::from("")),
            done: ed_request.done.unwrap_or(false),
            description: ed_request.description.unwrap_or(String::from("")),
            account_id,
        }
    }
    fn receive(&mut self, changes: TODOElementDataRequest) {
        if changes.title.is_some() {
            self.title = changes.title.unwrap();
        }
        if changes.done.is_some() {
            self.done = changes.done.unwrap();
        }
        if changes.description.is_some() {
            self.description = changes.description.unwrap();
        }
    }
}

#[derive(Deserialize, Serialize)]
struct TODOElementDataRequest {
    title: Option<String>,
    done: Option<bool>,
    description: Option<String>,
}
impl From<TODOElementData> for TODOElementDataRequest {
    fn from(value: TODOElementData) -> Self {
        Self {
            title: Some(value.title),
            done: Some(value.done),
            description: Some(value.description),
        }
    }
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
        .bind(todo.title.as_ref().map_or("", |s| s.as_ref()))
        .bind(todo.done)
        .bind(todo.description.as_ref().map_or("", |s| s.as_ref()))
        .bind(user_id_from_cookie)
        .fetch_one(&db_pool).await;
    if q.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    return HttpResponse::Ok()
        .json(TODOElementData::new(q.unwrap().get("id"), todo.into_inner(), user_id_from_cookie));
}

#[patch("/api/todos/{id}")]
pub async fn patch_api_todos_id(req: HttpRequest, path_params: Path<i32>, mod_todo: Json<TODOElementDataRequest>, ThinData(db_pool): ThinData<PgPool>) -> HttpResponse {
    let todo_id: i32 = path_params.into_inner();

    let q = sqlx::query_as::<_, TODOElementData>("SELECT * FROM todo_element_data WHERE id = $1;")
        .bind(todo_id)
        .fetch_one(&db_pool).await;
    if q.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let mut todo = q.unwrap();
    let todo_account_id: Option<i32> = todo.account_id;
    if todo_account_id.is_some() {
        let user_id_from_cookie: Option<i32> = match req.cookie("userId") {
            Some(cookie) => match cookie.value().parse() {
                Ok(user_id) => Some(user_id),
                Err(_) => None,
            }
            None => None,
        };
        if user_id_from_cookie != todo_account_id {
            return HttpResponse::Forbidden().finish();
        }
    }

    todo.receive(mod_todo.into_inner());
    return match sqlx::query("UPDATE todo_element_data SET title = $1, done = $2, description = $3 WHERE id = $4;")
        .bind(<String as AsRef<str>>::as_ref(&todo.title))
        .bind(todo.done)
        .bind(&todo.description)
        .bind(todo_id)
        .execute(&db_pool).await {
        Ok(_) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

#[delete("/api/todos/{id}")]
pub async fn delete_api_todos_id(req: HttpRequest, path_params: Path<i32>, ThinData(db_pool): ThinData<PgPool>) -> HttpResponse {
    let todo_id: i32 = path_params.into_inner();

    let q = sqlx::query("SELECT account_id FROM todo_element_data WHERE id = $1;")
        .bind(todo_id)
        .fetch_one(&db_pool).await;
    if q.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let q = q.unwrap();
    if q.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let todo_account_id: Option<i32> = q.get("account_id");
    if todo_account_id.is_some() {
        let user_id_from_cookie: Option<i32> = match req.cookie("userId") {
            Some(cookie) => match cookie.value().parse() {
                Ok(user_id) => Some(user_id),
                Err(_) => None,
            }
            None => None,
        };
        if user_id_from_cookie != todo_account_id {
            return HttpResponse::Forbidden().finish();
        }
    }

    return match sqlx::query("DELETE FROM todo_element_data WHERE id = $1;")
        .execute(&db_pool).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}
