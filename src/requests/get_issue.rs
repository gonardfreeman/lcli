use crate::config::constants::{LINEAR_URL, USER_AGENT};
use crate::models::issue::{GetIssue, get_issue, get_issue::Variables};

use anyhow::*;
use graphql_client::reqwest::post_graphql_blocking;
use log::*;
use prettytable::*;
use reqwest::blocking::Client;

pub fn fetch_linear_issue(issue_key: &String) -> Result<(), anyhow::Error> {
    let linear_api_token =
        std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");
    println!("issue_key: {issue_key:?}");
    let variables = Variables {
        issue_id: issue_key.to_string(),
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
        post_graphql_blocking::<GetIssue, _>(&req_client, LINEAR_URL, variables).unwrap();
    info!("{:?}", response_body.data);

    let response_data: get_issue::ResponseData = response_body.data.expect("no response body");
    let issue = response_data.issue;
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!(b => "id", "title" , "description"));
    table.add_row(row!(
        issue.id,
        issue.title.to_string(),
        issue.description.unwrap()
    ));

    table.printstd();
    Ok(())
}
