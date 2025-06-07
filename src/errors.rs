// Copyright (c) {{owner}}. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;
use tracing::error;

pub type Result<T, E = ApiError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[error("Not Found Workflow: {0}")]
    NotFoundWorkflow(String),

    #[error("Failed to create workflow: {0}")]
    FailedToCreateWorkflow(String),

    #[error("Failed to delete workflow: {0}")]
    FailedToDeleteWorkflow(String),

    #[error("Bad Workflow Request: {0}")]
    BadWorkflowRequest(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Self::NotFoundWorkflow(e) => (StatusCode::NOT_FOUND, e.to_string()),
            Self::FailedToCreateWorkflow(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::FailedToDeleteWorkflow(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::BadWorkflowRequest(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        error!("{} - {}", status, message);
        (status, Json(json!({ "message": message }))).into_response()
    }
}
