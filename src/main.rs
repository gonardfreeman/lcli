mod clients;
mod config;
mod models;
mod utils;

use anyhow::{Error as AnyError, Ok as AnyOk, anyhow};
use pico_args::Arguments;

use crate::clients::{cli_client::Commands, linear_client::LinearClient};

#[derive(Debug)]
struct Args {
    flag_help: bool,
    arg_inputs: String,
    flag_version: bool,
    flag_dont_subscribe: bool,
}

fn parse_args(args: &mut Arguments) -> Result<Args, AnyError> {
    AnyOk(Args {
        flag_help: args.contains(["-h", "--help"]),
        flag_version: args.contains(["-v", "--version"]),
        flag_dont_subscribe: args.contains(["-s", "--dont-subscribe"]),
        arg_inputs: args.free_from_str()?,
    })
}

fn main() -> Result<(), AnyError> {
    env_logger::init();
    let mut pargs = Arguments::from_env();
    let args = parse_args(&mut pargs)?;

    let linear_api_token =
        std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");
    let linear_client = LinearClient::new(&linear_api_token);

    let commander = Commands::new(&args, pargs, &linear_client);

    commander.execute()
    // let cli = Lcli::parse();
    // match &cli.command {
    //     Commands::Get { issue_key } => {
    //         let linear_issue = linear_client.get_comment(issue_key);
    //         match linear_issue {
    //             Ok(issue) => print_linear_results(&LinearResponseData::GetIssueIssue(issue)),
    //             Err(error) => println!("Error getting issue: {error:?}"),
    //         }
    //     }
    //     Commands::PostComment {
    //         issue_key,
    //         body,
    //         dont_subscribe,
    //     } => {
    //         let comment = linear_client.post_comment(issue_key, body, dont_subscribe);
    //         match comment {
    //             Ok(comment_response) => print_linear_results(
    //                 &LinearResponseData::PostCommentCommentCreate(comment_response),
    //             ),
    //             Err(error) => println!("Error posting comment: {error:?}"),
    //         }
    //     }
    // }
    // AnyOk(())
}
