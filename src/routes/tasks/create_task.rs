use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

use crate::{
    database::{tasks, users::Model as UserModel},
    utils::app_error::AppError,
};

use super::{create_task_extractor::ValidateCreateTask, ResponseDataTask, ResponseTask};

pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        is_default: Set(None),
        ..Default::default()
    };

    let task = new_task
        .save(&db)
        .await
        .map_err(|_error| {
            eprintln!("Error creating a new task");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?
        .try_into_model()
        .map_err(|_error| {
            eprintln!("Error converting task after creating it");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error creating task")
        })?;
    let response = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };
    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask { data: response }),
    ))
}
