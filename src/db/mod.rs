use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
    let task = models::NewTask { title, done: 0 };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn done_update_task(connection: &SqliteConnection, id: i32) {
    use super::db::schema::task::dsl::{task, done};

    let _ = diesel::update(task.find(id))
        .set(done.eq(1))
        .execute(connection)
        .unwrap_or_else(|_| panic!("Unable to find task {}", id));

    let done_task: models::Task = task
            .find(id)
            .first(connection)
            .unwrap_or_else(|_| panic!("Unable to find post {}", id));
    println!("Done task: {}", done_task.title);
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(&*connection)
        .expect("Error loading tasks")
}
