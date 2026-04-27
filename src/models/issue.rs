use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetIssueResponse {
    pub issue: Issue,
}
