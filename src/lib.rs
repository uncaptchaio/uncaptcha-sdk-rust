use serde::Serialize;

use crate::{
    api::{APIResponse, ExecuteTaskPayload, ExecuteTaskResponse, GetBalanceResponse},
    task::Task,
};

mod api;
pub mod errors;
pub mod task;

pub type Result<T> = std::result::Result<T, errors::Error>;

pub struct UncaptchaAPI {
    /// Client used to send API requests
    client: reqwest::Client,

    /// uncaptcha.io api key - can be found in dashboard
    api_key: String,
}

impl UncaptchaAPI {
    pub fn new(api_key: impl ToString) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            api_key: api_key.to_string(),
        }
    }

    pub async fn execute_task<T>(&self, task: T) -> Result<T::Solution>
    where
        T: Task + Serialize,
    {
        let payload = ExecuteTaskPayload {
            task_type: task.get_name(),
            task_data: task,
        };

        let r = self
            .client
            .post("https://api.uncaptcha.io/v1/task/execute")
            .header("Content-Type", "application/json")
            .header("X-Api-Key", &self.api_key)
            .json(&payload)
            .send()
            .await?;

        let response: APIResponse<ExecuteTaskResponse<T::Solution>> = r.json().await?;
        if !response.success {
            let error = match response.message {
                Some(message) => message,
                None => unreachable!(),
            };

            return Err(errors::Error::API(error));
        }

        let data = match response.data {
            Some(data) => data,
            None => unreachable!(),
        };

        Ok(data.solution)
    }

    pub async fn get_balance(&self) -> Result<f64> {
        let r = self
            .client
            .get("https://api.uncaptcha.io/v1/balance/get")
            .header("X-Api-Key", &self.api_key)
            .send()
            .await?;

        let response: APIResponse<GetBalanceResponse> = r.json().await?;
        if !response.success {
            let error = match response.message {
                Some(message) => message,
                None => unreachable!(),
            };

            return Err(errors::Error::API(error));
        }

        let data = match response.data {
            Some(data) => data,
            None => unreachable!(),
        };

        Ok(data.balance)
    }
}
