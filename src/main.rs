mod clients;
mod constants;
mod models;
mod utils;

use anyhow::{Error as AnyError, Ok as AnyOk};
use pico_args::Arguments;

use crate::clients::{cli::Commands, network::NetworkClient};

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
    let mut pargs = Arguments::from_env();
    let args = parse_args(&mut pargs)?;

    let linear_client = NetworkClient::new();

    let commander = Commands::new(&args, pargs, &linear_client);

    commander.execute()
}
