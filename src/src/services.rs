use actix_web::{
    web::{
        scope,
        Json,
        Path,
        Data,
        ServiceConfig,
        Query
    },
    get,
    post,
    delete,
    patch,
    HttpResponse,
    Responder,
};

use serde_json::json;
use uuid::Uuid;

use crate::{schema::{CreateTaskSchema, FilterOptions}, model::TaskModel, AppState};

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "api is running on port :3030";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[post("/task")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {

    match
        sqlx::query_as!(
            TaskModel,
            "INSERT INTO tasks (title, content) VALUES ($1, $2)
            RETURNING * ",
            body.title.to_string(),
            body.content.to_string()
        )
        .fetch_one(&data.db)
        .await {
            Ok(task) => {
                let note_response: serde_json::Value = json!({
                    "status": "success",
                    "task": json!(task)
                });

                return HttpResponse::Ok().json(note_response);
            }
            Err(error) => {

                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{:?}", error)
                    })
                )
            }

        }

}


#[get("/tasks")]
async fn get_all_tasks(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder{

    let limit: usize   = opts.limit.unwrap_or(10);
    let offset: usize = (opts.page.unwrap_or(1)-1)*limit;

    match
        sqlx::query_as!(
            TaskModel,
            "SELECT * FROM tasks ORDER by id LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32
        )
        .fetch_all(&data.db)
        .await{
            Ok(tasks) => {
                let json_response: serde_json::Value = json!({
                    "status": "success",
                    "result": tasks.len(),
                    "tasks": tasks
                });
                return HttpResponse::Ok().json(json_response);
            }
            Err(error) => {
                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{:?}", error)
                    })
                )
            }
        }

}

#[get("/tasks/{id}")]
async fn get_task_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {

    let task_id: Uuid = path.into_inner();

    match
        sqlx::query_as!(
            TaskModel,
            "SELECT * FROM tasks WHERE id = $1",
            task_id
        )
        .fetch_one(&data.db)
        .await {
            Ok(task) => {
                println!("task_id: {:?}", task_id);
                let task_note = json!({
                    "status": "success",
                    "task": task
                });

                return HttpResponse::Ok().json(task_note);
            }
            Err(error) => {
                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{:?}", error)
                    })
                )
            }
        }   
}


#[delete("/tasks/{id}")]
async fn delete_task_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {

    let task_id: Uuid = path.into_inner();

    match
        sqlx::query_as!(
            TaskModel,
            "DELETE FROM tasks WHERE id = $1",
            task_id
        )
        .execute(&data.db)
        .await {
            Ok(_) => {
                return HttpResponse::NoContent().finish();
            }
            Err(error) => {
                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{:?}", error)
                    })
                )
            }
        }   
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
    .service(health_checker)
    .service(create_task)
    .service(get_all_tasks)
    .service(get_task_by_id)
    .service(delete_task_by_id);
    conf.service(scope);


}