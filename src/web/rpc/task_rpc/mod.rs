use super::{ParamsForCreate, ParamsForIded, ParamsForUpdate};
use crate::model::task::{TaskBmc, TaskForUpdate};
use crate::web::Result;
use crate::{
    ctx::Ctx,
    model::{
        task::{Task, TaskForCreate},
        ModelManager,
    },
};

pub async fn create_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<TaskForCreate>,
) -> Result<Task> {
    let ParamsForCreate { data } = params;

    let id = TaskBmc::create(&ctx, &mm, data).await?;
    let task = TaskBmc::get(&ctx, &mm, id).await?;
    Ok(task)
}

pub async fn list_tasks(ctx: Ctx, mm: ModelManager) -> Result<Vec<Task>> {
    let tasks = TaskBmc::list(&ctx, &mm).await?;
    Ok(tasks)
}

pub async fn update_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<TaskForUpdate>,
) -> Result<Task> {
    let ParamsForUpdate { id, data } = params;

    TaskBmc::update(&ctx, &mm, id, data).await?;
    let task = TaskBmc::get(&ctx, &mm, id).await?;
    Ok(task)
}

pub async fn delete_task(ctx: Ctx, mm: ModelManager, params: ParamsForIded) -> Result<Task> {
    let ParamsForIded { id } = params;

    let task = TaskBmc::get(&ctx, &mm, id).await?;
    TaskBmc::delete(&ctx, &mm, id).await?;
    Ok(task)
}
