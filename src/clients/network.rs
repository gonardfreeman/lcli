use anyhow::{Error as AnyError, anyhow as any_macro};
use ureq::{Agent, Error::StatusCode as UStatus};

use crate::{
    constants::{
        graph_ql::{GET_ISSUE, POST_COMMENT},
        network::LINEAR_URI,
    },
    models::{
        comment::{CreateCommentInput, PostCommentInput},
        linear::{GetLinearIssueVariables, LinearRequest},
    },
};

pub struct NetworkClient {
    host_uri: &'static str,
    headers: Vec<(String, String)>,
}

impl NetworkClient {
    pub fn new() -> Self {
        let linear_api_key =
            std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");
        Self {
            host_uri: LINEAR_URI,
            headers: vec![
                ("Content-Type".into(), "application/json".into()),
                ("Authorization".into(), linear_api_key),
            ],
        }
    }

    fn make_call(&self, body: &String) -> Result<String, AnyError> {
        let agent: Agent = Agent::config_builder()
            .http_status_as_error(false)
            .build()
            .into();
        let mut req = agent.post(self.host_uri);
        for (k, v) in &self.headers {
            req = req.header(k, v);
        }
        let res = req.send(body)?;
        match res.into_body().read_to_string() {
            Ok(data) => Ok(data),
            Err(UStatus(code)) => Err(any_macro!("Failed with code: \n{}", code)),
            Err(error) => Err(any_macro!("Error message: {}", error)),
        }
    }

    pub fn get_issue(&self, issue_key: &String) -> Result<String, AnyError> {
        let variables = GetLinearIssueVariables { issue_key };
        let body = LinearRequest {
            query: GET_ISSUE,
            variables: &variables,
        };

        let json_body = serde_json::to_string(&body).expect("Can't parse JSON");
        self.make_call(&json_body)
    }

    pub fn post_comment(
        &self,
        issue_key: &String,
        body: &String,
        dont_subscribe: &bool,
    ) -> Result<String, AnyError> {
        let variables = CreateCommentInput {
            // replace owned with references
            input: PostCommentInput {
                body: Some(body.to_owned()),
                issue_id: issue_key.to_owned(),
                do_not_subscribe_to_issue: dont_subscribe.to_owned(),
            },
        };

        let body = LinearRequest {
            query: POST_COMMENT,
            variables: &variables,
        };

        let json_body = serde_json::to_string(&body).expect("Can't parse JSON");
        self.make_call(&json_body)
    }
}
