use loco_rs::prelude::*;
use serde::Serialize;

use crate::models::index_tasks;

#[derive(Serialize)]
pub struct IndexResponse {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub path: String,
    pub progress: f32,
    pub queue: String,
}
impl From<index_tasks::Model> for IndexResponse {
    fn from(value: index_tasks::Model) -> Self {
        Self {
            created_at: value.created_at,
            updated_at: value.updated_at,
            id: value.id,
            path: value.path,
            progress: value.progress,
            queue: value.queue,
        }
    }
}
