use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTaskSchema{
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FilterOptions{
    pub page: Option<usize>,
    pub limit: Option<usize>,
}