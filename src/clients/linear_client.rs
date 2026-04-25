use crate::config::constants::{LINEAR_URL, USER_AGENT};
use crate::models::comment::{
    PostComment,
    post_comment::{
        CommentCreateInput, PostCommentCommentCreate, ResponseData as PostCommentResponseData,
        Variables as PostCommentVariables,
    },
};
use crate::models::issue::{
    GetIssue,
    get_issue::{GetIssueIssue, ResponseData as GetIssueResponse, Variables as GetIssueVariables},
};

use anyhow::{Error as AnyError, Ok as AnyOk, anyhow as any_macro};
use graphql_client::reqwest::post_graphql_blocking;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT as USER_AGENT_KEY};

pub struct LinearClient {
    http: Client,
}

impl LinearClient {
    fn build_headers(api_key: &str) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert(USER_AGENT_KEY, HeaderValue::from_str(USER_AGENT).unwrap());
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(api_key).unwrap());

        header_map
    }

    pub fn new(api_key: &str) -> Self {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(Self::build_headers(api_key))
            .build()
            .expect("Can't build client");
        Self { http: client }
    }

    pub fn get_issue(&self, issue_key: &String) -> Result<GetIssueIssue, AnyError> {
        let variables = GetIssueVariables {
            issue_id: issue_key.to_string(),
        };
        let response_body =
            post_graphql_blocking::<GetIssue, _>(&self.http, LINEAR_URL, variables).unwrap();

        if let Some(err) = response_body.errors {
            println!(
                "Failed with:\n{}",
                serde_json::to_string_pretty(&err).unwrap()
            )
        }
        let response_data: GetIssueResponse = response_body.data.expect("No response data");
        AnyOk(response_data.issue)
    }

    pub fn post_comment(
        &self,
        issue_key: &String,
        body: &String,
        dont_subscribe: &Option<bool>,
    ) -> Result<PostCommentCommentCreate, AnyError> {
        let variables = PostCommentVariables {
            input: CommentCreateInput {
                body: Some(body.to_string()),
                issue_id: Some(issue_key.to_string()),
                do_not_subscribe_to_issue: dont_subscribe.to_owned(),
                body_data: None,
                create_as_user: None,
                create_on_synced_slack_thread: None,
                created_at: None,
                display_icon_url: None,
                document_content_id: None,
                id: None,
                initiative_id: None,
                parent_id: None,
                post_id: None,
                initiative_update_id: None,
                project_id: None,
                project_update_id: None,
                quoted_text: None,
                subscriber_ids: None,
            },
        };

        let response_body =
            post_graphql_blocking::<PostComment, _>(&self.http, LINEAR_URL, variables).unwrap();
        if let Some(err) = response_body.errors {
            println!(
                "Failed with:\n{}",
                serde_json::to_string_pretty(&err).unwrap()
            );
            return Err(any_macro!(
                "Failed to post a comment to linear: {issue_key:}",
            ));
        }
        let response_data: PostCommentResponseData = response_body.data.unwrap();

        AnyOk(response_data.comment_create)
    }
}
