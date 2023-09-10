use crate::ctx::Ctx;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use super::base::{self, DbBmc};
use super::ModelManager;

#[derive(Clone, Debug, Fields, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

pub struct TaskBmc {}

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task: TaskForCreate) -> Result<i64> {
        base::create::<Self, _>(_ctx, mm, task).await
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(_ctx, mm, id).await
    }

    pub async fn update(_ctx: &Ctx, mm: &ModelManager, id: i64, task: TaskForUpdate) -> Result<()> {
        base::update::<Self, _>(_ctx, mm, id, task).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(_ctx, mm, id).await
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        base::list::<Self, _>(_ctx, mm).await
    }
}

// #[cfg(test)]
// mod tests {
//     #![allow(unused)]
//     use crate::_dev_utils;

//     use super::*;
//     use anyhow::Result;
//     use serial_test::serial;

//     #[serial]
//     #[tokio::test]
//     async fn test_create_ok() -> Result<()> {
//         let mm = _dev_utils::init_test().await;
//         let ctx = Ctx::root_ctx();
//         let fx_title = "test_create_ok title";

//         let task_c = TaskForCreate {
//             title: fx_title.to_string(),
//         };
//         let id = TaskBmc::create(&ctx, &mm, task_c).await?;

//         let (title,): (String,) = sqlx::query_as("SELECT title FROM task WHERE id = $1")
//             .bind(id)
//             .fetch_one(mm.db())
//             .await?;
//         println!("--> {:?}", title);
//         println!("--> {:?}", fx_title);
//         assert_eq!(title, fx_title);
//         let count = sqlx::query("DELETE FROM task WHERE id = $1")
//             .bind(id)
//             .execute(mm.db())
//             .await?
//             .rows_affected();
//         assert_eq!(count, 1, "Did not delete 1 row");

//         Ok(())
//     }

//     #[serial]
//     #[tokio::test]
//     async fn test_get_err_not_found() -> Result<()> {
//         let mm = _dev_utils::init_test().await;
//         let ctx = Ctx::root_ctx();
//         let fx_id = 100;
//         println!("--> 1111");

//         let res = TaskBmc::get(&ctx, &mm, fx_id).await;
//         println!("--> {:?}", res);
//         assert!(
//             matches!(
//                 res,
//                 Err(Error::EntityNotFound {
//                     entity: "task",
//                     id: 100
//                 })
//             ),
//             "EntityNotFound not matching"
//         );

//         Ok(())
//     }

//     #[serial]
//     #[tokio::test]
//     async fn test_list_ok() -> Result<()> {
//         let mm = _dev_utils::init_test().await;
//         let ctx = Ctx::root_ctx();
//         let fx_titles = &["test_title_1", "test_title_2"];

//         println!("--> fx_titles {:?}", fx_titles);
//         _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;
//         println!("--> 111111");

//         let tasks = TaskBmc::list(&ctx, &mm).await?;
//         println!("--> tasks {:?}", tasks);
//         let tasks: Vec<Task> = tasks
//             .into_iter()
//             .filter(|t| t.title.starts_with("test_title"))
//             .collect();
//         println!("--> {:?}", tasks);
//         println!("--> {}", tasks.len());
//         assert_eq!(tasks.len(), 2);

//         for task in tasks.iter() {
//             TaskBmc::delete(&ctx, &mm, task.id).await?;
//         }

//         Ok(())
//     }
// }
