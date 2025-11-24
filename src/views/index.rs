use std::{convert::Infallible, fmt::Display, str::FromStr};

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::index_tasks;

#[derive(Serialize)]
pub struct IndexResponse {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub status: IndexStatus,
    pub path: String,
    pub progress: f32,
    pub queue: String,
}
impl From<index_tasks::Model> for IndexResponse {
    fn from(value: index_tasks::Model) -> Self {
        let Ok(status) = IndexStatus::from_str(&value.status);
        Self {
            created_at: value.created_at,
            updated_at: value.updated_at,
            id: value.id,
            status,
            path: value.path,
            progress: value.progress,
            queue: value.queue,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndexStatus {
    InProgress,
    Complete,
    Cancelled,
}

impl Display for IndexStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(f, "cancelled"),
            Self::Complete => write!(f, "complete"),
            Self::InProgress => write!(f, "in_progress"),
        }
    }
}
impl FromStr for IndexStatus {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cancelled" => Ok(Self::Cancelled),
            "complete" => Ok(Self::Complete),
            "in_progress" => Ok(Self::InProgress),
            _ => panic!("Invalid status"),
        }
    }
}
