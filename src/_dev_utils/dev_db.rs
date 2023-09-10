use std::{error::Error, fs, path::PathBuf, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::{
    ctx::Ctx,
    model::{
        user::{User, UserBmc},
        ModelManager,
    },
};

type Db = Pool<Postgres>;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

//sql files

const SQL_RECREATE_DB: &str = "sql/dev_initial/00_recreate_db.sql";
const SQL_DIR: &str = "sql/dev_initial";

const DEMO_PWD: &str = "welcome";

pub async fn init_dev_db() -> Result<(), Box<dyn Error>> {
    info!("{:<12} - init_dev_db", "FOR-DEV-ONLY");

    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, SQL_RECREATE_DB).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace('\\', "/");
            info!("{:<12} - path: {path}", "FOR-DEV-ONLY");

            // only take the .sql and skip SQL_RECREATE_DB
            if path.ends_with(".sql") && path != SQL_RECREATE_DB {
                pexec(&app_db, &path).await?;
            }
        }
    }

    let mm = ModelManager::new().await?;
    let ctx = Ctx::root_ctx();

    let demo_user1: User = UserBmc::first_by_username(&ctx, &mm, "demo1")
        .await?
        .unwrap();
    UserBmc::update_pwd(&ctx, &mm, demo_user1.id, DEMO_PWD).await?;
    info!("{:<12} - init_dev_db - set demo1 pwd", "FOR-DEV-ONLY");
    Ok(())
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file}", "FOR-DEV-ONLY");

    let content = fs::read_to_string(file)?;

    //fixme: prepare it better
    let sqls: Vec<&str> = content.split(";").collect();
    for sql in sqls {
        println!("{:<12} - sqls: {:?}", "FOR-DEV-ONLY", sql);
        sqlx::query(sql).execute(db).await?;
    }

    Ok(())
}

async fn new_db_pool(db_conn_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_conn_url)
        .await
}
