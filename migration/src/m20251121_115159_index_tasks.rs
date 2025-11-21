use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "index_tasks",
            &[
            
            ("id", ColType::PkAuto),
            
            ("path", ColType::String),
            ("progress", ColType::Integer),
            ("queue", ColType::Json),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "index_tasks").await
    }
}
