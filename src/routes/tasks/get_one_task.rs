use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::DatabaseConnection;

use crate::{database::users::Model, utils::app_error::AppError};
use crate::{queries::task_queries, routes::tasks::ResponseTask};

use super::ResponseDataTask;

pub async fn get_one_task(
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Path(task_id): Path<i32>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = task_queries::find_task_by_id(&db, task_id, user.id).await?;

    let response_task = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task
            .completed_at
            .map(|completed_at| completed_at.to_string()),
    };

    Ok(Json(ResponseDataTask {
        data: response_task,
    }))
}
