use todo_rs::db::{query_task, establish_connection};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().body("index\n")
}

fn tasks_get() -> impl Responder {
    let mut response: Vec<String> = vec![];

    let conn = establish_connection();
    for task in query_task(&conn) {
        response.push(task.title);
    }
    HttpResponse::Ok().body(response.join("\n"))
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
