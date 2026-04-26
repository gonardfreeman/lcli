mod clients;
mod config;
mod constants;
mod models;
mod utils;

use anyhow::{Error as AnyError, Ok as AnyOk};
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
    // env_logger::init();
    let mut pargs = Arguments::from_env();
    let args = parse_args(&mut pargs)?;

    let linear_api_token =
        std::env::var("LINEAR_API_KEY").expect("Missing LINEAR_API_KEY in your env");
    let linear_client = LinearClient::new(&linear_api_token);

    let commander = Commands::new(&args, pargs, &linear_client);

    commander.execute()
}
