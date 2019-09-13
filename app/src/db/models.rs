use super::schema::task;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub done: i32,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: i32,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    pub data: Vec<TaskJson>,
}

#[derive(Serialize)]
pub struct TaskJson {
    pub id: i32,
    #[serde(rename = "type")]
    pub type_json: String,
    pub attributes: TaskAttributes
}

impl TaskJson {
    pub fn new(task: Task) -> TaskJson {
        TaskJson {
            id: task.id,
            type_json: "Task".into(),
            attributes: TaskAttributes { title: task.title, done: task.done}
        }
    }
}

#[derive(Serialize)]
pub struct TaskAttributes {
    pub title: String,
    pub done: i32,
}
