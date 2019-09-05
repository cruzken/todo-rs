use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use std::ops::Deref;
use todo_rs::db::{
    get_connect, init_pool,
    models::{JsonApiResponse, TaskJson},
    query_task, SqlitePool,
};

fn index() -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let result = HttpResponse::Ok().body("test");
        if result.status().is_success() {
            Ok("yup")
        } else {
            Err("uh oh")
        }
    })
    .then(move |res| match res {
        Ok(_) => Ok(HttpResponse::Ok().body("test\n")),
        Err(_) => Ok(HttpResponse::Ok().body("unknown")),
    })
}

fn tasks_get(pool: web::Data<SqlitePool>) -> impl Future<Item = HttpResponse, Error = Error> {
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
            .service(web::resource("/tasks").route(web::get().to_async(tasks_get)))
    };

    println!("Starting server");
    HttpServer::new(app)
        .bind("localhost:8088")
        .unwrap()
        .run()
        .unwrap();
}
