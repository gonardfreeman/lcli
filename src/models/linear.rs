use serde::{Deserialize, Serialize};

use crate::models::{comment::PostCommentResponse, issue::GetIssueResponse};

#[derive(Serialize, Debug)]
pub struct LinearRequest<'a, T> {
    pub query: &'a str,
    pub variables: &'a T,
}

#[derive(Serialize, Debug)]
pub struct GetLinearIssueVariables<'a> {
    pub issue_key: &'a String,
}

#[derive(Deserialize, Debug)]
pub struct LinearResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<LinearError>>,
}

#[derive(Deserialize, Debug)]
pub struct LinearError {
    pub message: String,
    pub extensions: Option<LinearErrorExtensions>,
}

#[derive(Deserialize, Debug)]
pub struct LinearErrorExtensions {
    pub code: String,
    #[serde(rename = "userPresentableMessage")]
    pub user_message: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}

pub enum LinearData {
    GetIssueIssue(GetIssueResponse),
    PostCommentCommentCreate(PostCommentResponse),
}
