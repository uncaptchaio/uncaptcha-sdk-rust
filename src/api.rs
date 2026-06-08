use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Deserialize)]
pub struct APIResponse<T> {
    pub(crate) success: bool,
    pub(crate) message: Option<String>,
    pub(crate) data: Option<T>,
}

#[derive(Serialize)]
pub struct ExecuteTaskPayload<T: Task + Serialize> {
    pub(crate) task_type: &'static str,
    pub(crate) task_data: T,
}

#[derive(Deserialize)]
pub struct GetBalanceResponse {
    pub(crate) balance: f64,
}

#[derive(Deserialize)]
pub struct ExecuteTaskResponse<T> {
    pub(crate) solution: T,
}
