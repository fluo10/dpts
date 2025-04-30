use std::time::Duration;
use chrono::{offset, FixedOffset, Local, TimeZone};
use sea_orm::{entity::*, query::*, ConnectOptions, Database};
use dpts_migration::{Migrator, MigratorTrait};
use dpts_entity::*;

#[tokio::test]
async fn main() {
    let mut opt = ConnectOptions::new("sqlite::memory:");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);
        //.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    let db= Database::connect(opt).await.unwrap();
    Migrator::fresh(&db).await.unwrap();

    let local_date_time = Local::now();
    let offset_date_time = local_date_time.with_timezone(local_date_time.offset());

    let user = UserActiveModel{
        login_name: Set("admin".to_owned()),
        password_hash: Set("admin".to_owned()),
        created_at: Set(offset_date_time),
        updated_at: Set(offset_date_time),
        ..Default::default()
    }.insert(&db)
    .await.unwrap();


    let record_tag = RecordTagActiveModel{
        user_id: Set(user.id),
        name: Set("test".to_owned()),
        ..Default::default()
    }.insert(&db)
    .await.unwrap();

    let record_header = RecordHeaderActiveModel{
        user_id: Set(user.id),
        created_at: Set(offset_date_time),
        updated_at: Set(offset_date_time),
        recorded_at: Set(offset_date_time),
        comment: Set("".to_owned()),
        ..Default::default()
    }.insert(&db)
    .await.unwrap();

    RecordDetailActiveModel {
        record_header_id: Set(record_header.id),
        record_tag_id: Set(record_tag.id),
        count: Set(1),
        ..Default::default()
    }.insert(&db)
    .await.unwrap();
    RecordDetailActiveModel {
        record_header_id: Set(record_header.id),
        record_tag_id: Set(record_tag.id),
        count: Set(2),
        ..Default::default()
    }.insert(&db)
    .await.unwrap();


    Migrator::reset(&db).await.unwrap();
    db.close().await.unwrap();
}