mod config;
mod models;
mod requests;

use anyhow::*;
use clap::{Parser, Subcommand};

use crate::requests::{get_issue::fetch_linear_issue, post_comment::post_linear_comment};

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
    let cli = Lcli::parse();

    match &cli.command {
        Commands::Get { issue_key } => {
            let _ = fetch_linear_issue(issue_key);
        }
        Commands::PostComment {
            issue_key,
            body,
            dont_subscribe,
        } => {
            let _ = post_linear_comment(issue_key, body, dont_subscribe);
        }
    }

    Ok(())
}
