use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use todo_rs::db::{establish_connection, models::Task, query_task};

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Task>,
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("index\n")
}

fn tasks_get() -> impl Responder {
    let mut response = JsonApiResponse { data: vec![] };
    let conn = establish_connection();
    for task in query_task(&conn) {
        response.data.push(task);
    }
    HttpResponse::Ok().json(response)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/tasks", web::get().to(tasks_get))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
