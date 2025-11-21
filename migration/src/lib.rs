#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20251121_101838_searches;
mod m20251121_102944_files;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251121_101838_searches::Migration),
            Box::new(m20251121_102944_files::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}