use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use serde::Deserialize;
use std::ops::Deref;
use todo_rs::db::{
    create_task, get_connect, init_pool,
    models::{JsonApiResponse, TaskJson},
    query_task, SqlitePool,
};

fn add_task(
    pool: web::Data<SqlitePool>,
    task: web::Json<JsonPostTask>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let pool = pool.clone();
    web::block(move || create_task(get_connect(&pool).unwrap().deref(), &task.title)).then(
        move |res| match res {
            Ok(_) => Ok(HttpResponse::Ok().body("task added")),
            Err(e) => Ok(HttpResponse::Ok().body(format!("error occured: {:?}", e))),
        },
    )
}

#[derive(Deserialize)]
pub struct JsonPostTask {
    pub title: String,
}

fn index(pool: web::Data<SqlitePool>) -> impl Future<Item = HttpResponse, Error = Error> {
    let pool = pool.clone();

    let mut response = JsonApiResponse { data: vec![] };

    web::block(move || query_task(get_connect(&pool).unwrap().deref()))
        .from_err()
        .then(move |res| match res {
            Ok(tasks) => {
                for task in tasks {
                    response.data.push(TaskJson::new(task));
                }
                return Ok(HttpResponse::Ok().json(response));
            }
            Err(e) => Err(e),
        })
}

fn main() {
    let pool = init_pool("./testdb.sqlite3").expect("Failed to create pool");
    let app = move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to_async(index)))
            .service(web::resource("/add").route(web::post().to_async(add_task)))
    };

    println!("Starting server");
    HttpServer::new(app)
        .bind("localhost:8088")
        .unwrap()
        .run()
        .unwrap();
}
