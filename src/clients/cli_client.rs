use anyhow::{Error as AnyError, Ok as AnyOk, anyhow as any_macro};
use pico_args::Arguments;

use crate::{
    Args,
    clients::linear_client::LinearClient,
    utils::{misc::convert_os_to_str, print_table::print_linear_results},
};

static VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_TEXT: &str = "
Usage: lcli [options] <inputs>...
       lcli --help
       lcli (-v|--version)

Inputs:

Options:
    -h, --help      Print help
    -v, --version   Print version
";

pub struct Commands<'a> {
    args: &'a Args,
    main: String,
    rest: Vec<String>,
    linear_client: &'a LinearClient,
}

impl<'a> Commands<'a> {
    pub fn new(args: &'a Args, pargs: Arguments, linear_client: &'a LinearClient) -> Self {
        Self {
            args,
            main: args.arg_inputs.to_owned(),
            // drop first, because it will set above
            rest: pargs
                .finish()
                .into_iter()
                .skip(1)
                .map(|s| convert_os_to_str(&s))
                .collect(),
            linear_client,
        }
    }
}

impl<'a> Commands<'a> {
    fn get_issue_key(&self) -> Result<&String, AnyError> {
        if let Some(issue_key) = self.rest.first() {
            Ok(issue_key)
        } else {
            Err(any_macro!("No issue key provided"))
        }
    }

    fn get_issue(&self) -> Result<(), AnyError> {
        match self.get_issue_key() {
            Ok(issue_key) => {
                let linear_issue = self.linear_client.get_issue(issue_key);
                match linear_issue {
                    Ok(issue) => {
                        print_linear_results(
                            &crate::utils::print_table::LinearResponseData::GetIssueIssue(issue),
                        );
                        AnyOk(())
                    }
                    Err(error) => Err(any_macro!("Error getting issue: {}", error)),
                }
            }
            Err(error) => Err(any_macro!("{}", error)),
        }
    }

    fn post_comment(&self) -> Result<(), AnyError> {
        match self.get_issue_key() {
            Ok(issue_key) => {
                println!("{}", issue_key);
                println!("rest: {:?}", self.rest);
                AnyOk(())
            }
            Err(error) => Err(any_macro!("{}", error)),
        }
    }
}

impl<'a> Commands<'a> {
    pub fn execute(&self) -> Result<(), AnyError> {
        if self.args.arg_inputs.is_empty() {
            return Err(any_macro!("no input args provided"));
        }
        if self.args.flag_help {
            println!("{}", HELP_TEXT);
            return AnyOk(());
        }

        if self.args.flag_version {
            println!("{}", VERSION);
            return AnyOk(());
        }

        match self.main.as_str() {
            "get" => self.get_issue(),
            "comment" => self.post_comment(),
            &_ => Err(any_macro!("Unknown param")),
        }
    }
}
