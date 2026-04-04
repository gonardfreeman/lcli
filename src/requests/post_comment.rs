use crate::config::constants::{LINEAR_URL, USER_AGENT};
use crate::models::comment::{PostComment, post_comment, post_comment::Variables};

use anyhow::*;
use graphql_client::reqwest::post_graphql_blocking;
use prettytable::*;
use reqwest::blocking::Client;
use serde_json::error;

pub fn post_linear_comment(issue_key: &String, body: &String) -> Result<(), anyhow::Error> {
    let linear_api_token =
        std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");

    println!("Posting comment: {}\nto issue: {}", issue_key, body);
    let variables = Variables {
        input: post_comment::CommentCreateInput {
            body: Some(body.to_string()),
            issue_id: Some(issue_key.to_string()),
            body_data: None,
            create_as_user: None,
            create_on_synced_slack_thread: None,
            created_at: None,
            display_icon_url: None,
            do_not_subscribe_to_issue: None,
            document_content_id: None,
            id: None,
            initiative_id: None,
            initiative_update_id: None,
            parent_id: None,
            post_id: None,
            project_id: None,
            project_update_id: None,
            quoted_text: None,
            subscriber_ids: None,
        },
    };

    let req_client = Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&linear_api_token).unwrap(),
            ))
            .collect(),
        )
        .build()?;
    let response_body =
        post_graphql_blocking::<PostComment, _>(&req_client, LINEAR_URL, variables).unwrap();

    if let Some(err) = response_body.errors {
        println!(
            "Failed with:\n{}",
            serde_json::to_string_pretty(&err).unwrap()
        );
        return Err(anyhow!("Failed to post a comment to linear: {issue_key:}"));
    }

    let response_data: post_comment::ResponseData = response_body.data.expect("no response body");
    let is_success = response_data.comment_create.success;
    let comment_data = response_data.comment_create.comment;
    println!("post comment: {:?}", if is_success { "✅" } else { "❌" });
    if !is_success {
        return Ok(());
    }
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "id", "issue", "body"));
    table.add_row(row!(
        comment_data.id,
        comment_data.issue_id.unwrap(),
        comment_data.body
    ));

    table.printstd();
    Ok(())
}
