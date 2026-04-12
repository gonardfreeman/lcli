mod config;
mod models;
mod requests;
mod utils;

use anyhow::{Error as AnyError, Ok as AnyOk};
use std::result::Result::Ok;

use clap::{Parser, Subcommand};

use crate::{
    requests::linear_client::LinearClient,
    utils::print_table::{LinearResponseData, print_linear_results},
};

#[derive(Parser, Debug)]
#[command(name = "Linear CLI")]
#[command(version = "1.0")]
#[command(about = "Simple Linear CLI", long_about = None)]
struct Lcli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Gets issue from linear
    Get { issue_key: String },

    /// Post comment to issue
    PostComment {
        #[arg(long, short)]
        issue_key: String,
        #[arg(long, short)]
        body: String,
        #[arg(long, short)]
        dont_subscribe: Option<bool>,
    },
}

fn main() -> Result<(), AnyError> {
    env_logger::init();

    let linear_api_token =
        std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");
    let linear_client = LinearClient::new(&linear_api_token);
    let cli = Lcli::parse();
    match &cli.command {
        Commands::Get { issue_key } => {
            let linear_issue = linear_client.get_comment(issue_key);
            match linear_issue {
                Ok(issue) => print_linear_results(&LinearResponseData::GetIssueIssue(issue)),
                Err(error) => println!("Error getting issue: {error:?}"),
            }
        }
        Commands::PostComment {
            issue_key,
            body,
            dont_subscribe,
        } => {
            let comment = linear_client.post_comment(issue_key, body, dont_subscribe);
            match comment {
                Ok(comment_response) => print_linear_results(
                    &LinearResponseData::PostCommentCommentCreate(comment_response),
                ),
                Err(error) => println!("Error posting comment: {error:?}"),
            }
        }
    }
    AnyOk(())
}
