mod config;
mod models;
mod requests;
mod utils;

use anyhow::*;

use clap::{Parser, Subcommand};

use crate::{
    models::issue::get_issue::GetIssueIssue,
    requests::{
        get_issue::fetch_linear_issue, linear_client::LinearClient,
        post_comment::post_linear_comment,
    },
    utils::{print, print_table::print_linear_results},
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

fn main() -> Result<(), Error> {
    env_logger::init();

    let linear_api_token =
        std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");
    let linear_client = LinearClient::new(&linear_api_token);

    let cli = Lcli::parse();

    match &cli.command {
        Commands::Get { issue_key } => {
            let linear_issue = linear_client.get_comment(issue_key);
            match linear_issue {
                Ok(issue) => {
                    print_linear_results(&issue);
                }
                Err(_) => {
                    println!("Error getting issue");
                }
            }
        }
        Commands::PostComment {
            issue_key,
            body,
            dont_subscribe,
        } => {
            let comment = linear_client.post_comment(issue_key, body, dont_subscribe);
            print_linear_results(&comment);
        }
    }

    Ok(())
}
