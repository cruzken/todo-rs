use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connect(pool: &SqlitePool) -> Result<SqlitePooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task<'a>(
    connection: &SqliteConnection,
    title: &'a str,
) -> Result<usize, diesel::result::Error> {
    let task = models::NewTask { title, done: 0 };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
}

pub fn done_update_task(connection: &SqliteConnection, id: i32) -> Result<String, String> {
    use super::db::schema::task::dsl::{done, task};

    match task.find(id).first::<models::Task>(connection) {
        Ok(t) => match diesel::update(task.find(id))
            .set(done.eq(1))
            .execute(connection)
        {
            Ok(_) => Ok(format!("task updated: {}", t.title)),
            Err(_) => Err("could not find task".into()),
        },
        Err(_) => Err("could not find task".into()),
    }
}

pub fn del_task(connection: &SqliteConnection, id: i32) -> Result<String, String> {
    use super::db::schema::task::dsl::task;

    match task.find(id).first::<models::Task>(connection) {
        Ok(t) => match diesel::delete(task.find(id)).execute(connection) {
            Ok(_) => Ok(format!("task deleted: {}", t.title)),
            Err(_) => Err("DB query error occured".into()),
        },
        Err(_) => Err("could not find task".into()),
    }
}

pub fn query_task(
    connection: &SqliteConnection,
) -> Result<Vec<models::Task>, diesel::result::Error> {
    schema::task::table.load::<models::Task>(&*connection)
}
