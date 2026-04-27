use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct PostCommentInput {
    pub body: Option<String>,
    #[serde(rename = "issueId")]
    pub issue_id: String,
    #[serde(rename = "doNotSubscribeToIssue")]
    pub do_not_subscribe_to_issue: bool,
}

#[derive(Serialize, Debug)]
pub struct CreateCommentInput {
    pub input: PostCommentInput,
}

#[derive(Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    #[serde(rename = "issueId")]
    pub issue_id: Option<String>,
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct PostCommentResponse {
    #[serde(rename = "lastSyncId")]
    pub last_sync_id: f64,
    pub success: bool,
    pub comment: Comment,
}
