use anyhow::{Error as AnyError, Ok as AnyOk, anyhow as any_macro};
use pico_args::Arguments;

use crate::{
    Args,
    clients::linear_client::LinearClient,
    constants::{
        errors::{
            ERROR_GETTING_ISSUE, ERROR_POSTING_COMMENT, MISSING_COMMENT_BODY, NO_INPUT_ARGS,
            NO_ISSUE_KEY, UNKNOWN_PARAM,
        },
        texts::{HELP_TEXT, VERSION},
    },
    utils::{misc::convert_os_to_str, print_table::print_linear_results},
};

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
            rest: pargs.finish().iter().map(convert_os_to_str).collect(),
            linear_client,
        }
    }
}

impl<'a> Commands<'a> {
    fn get_issue_key(&self) -> Result<&String, AnyError> {
        if let Some(issue_key) = self.rest.first() {
            Ok(issue_key)
        } else {
            Err(any_macro!(NO_ISSUE_KEY))
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
                    Err(error) => Err(any_macro!("{} {}", ERROR_GETTING_ISSUE, error)),
                }
            }
            Err(error) => Err(any_macro!("{}", error)),
        }
    }

    fn post_comment(&self) -> Result<(), AnyError> {
        match self.get_issue_key() {
            Ok(issue_key) => {
                println!("Issue: {}", issue_key);
                println!("Comment body: {:?}", self.rest);
                let comment_body = self
                    .rest
                    .get(1..)
                    .ok_or_else(|| any_macro!("{}", MISSING_COMMENT_BODY))?
                    .join(" ");
                let comment = self.linear_client.post_comment(
                    issue_key,
                    &comment_body,
                    &self.args.flag_dont_subscribe,
                );
                match comment {
                    Ok(comment_response) => print_linear_results(
                        &crate::utils::print_table::LinearResponseData::PostCommentCommentCreate(
                            comment_response,
                        ),
                    ),
                    Err(error) => println!("{} {}", ERROR_POSTING_COMMENT, error),
                }
                AnyOk(())
            }
            Err(error) => Err(any_macro!("{}", error)),
        }
    }
}

impl<'a> Commands<'a> {
    pub fn execute(&self) -> Result<(), AnyError> {
        if self.args.arg_inputs.is_empty() {
            return Err(any_macro!(NO_INPUT_ARGS));
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
            &_ => Err(any_macro!(UNKNOWN_PARAM)),
        }
    }
}
