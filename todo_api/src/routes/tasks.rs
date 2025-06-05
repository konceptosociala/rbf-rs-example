use crate::db::Database;
use crate::error::Result;
use crate::models::task::{Task, TaskResult};
use rocket::serde::json::{json, Json, Value};
use rocket::{get, post, State};
use rocket_okapi::openapi;

#[openapi(tag = "Tasks")]
#[get("/tasks")]
pub async fn tasks(db: &State<Database>) -> Result<Value> {
    let tasks: Vec<TaskResult> = db.select("tasks").await?;
    
    Ok(json!({"tasks":tasks}))
}

#[openapi(tag = "Tasks")]
#[post("/tasks", format = "json", data = "<task>")]
pub async fn tasks_create(db: &State<Database>, task: Json<Task>) -> Result<Value> {
    let created = db.insert("tasks", task.into_inner()).await?;
    
    Ok(json!(created))
}

#[openapi(tag = "Tasks")]
#[post("/tasks/delete/<id>")]
pub async fn tasks_delete(db: &State<Database>, id: &str) -> Value {
    let created = db.delete("tasks", id).await;

    match created {
        Ok(Some(_)) => {
            json!({"deleted": true})
        }
        _ => json!({"deleted": false}),
    }
}

#[openapi(tag = "Tasks")]
#[post("/tasks/update/<id>", format = "json", data = "<task>")]
pub async fn tasks_update(db: &State<Database>, id: &str, task: Json<Task>) -> Value {
    let updated = db.update("tasks", id, task.into_inner()).await;

    match updated {
        Ok(Some(_)) => {
            json!({"updated": true})
        }
        _ => json!({"updated": false}),
    }
}