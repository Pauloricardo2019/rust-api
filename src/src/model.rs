use actix_web::cookie::time::Date;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid; 

#[derive(Debug,Deserialize, Serialize, FromRow)]
pub struct TaskModel{
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl TaskModel{
    pub fn validate_task(&self)  {
        if self.title.is_empty(){
            
        }
    }



}