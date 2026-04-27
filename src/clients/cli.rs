use anyhow::{Error as AnyError, Ok as AnyOk, anyhow as any_macro};
use pico_args::Arguments;

use crate::{
    Args,
    clients::network::NetworkClient,
    constants::{
        errors::{MISSING_COMMENT_BODY, NO_INPUT_ARGS, NO_ISSUE_KEY, UNKNOWN_PARAM},
        texts::{HELP_TEXT, VERSION},
    },
    models::{
        comment::PostCommentResponse,
        issue::GetIssueResponse,
        linear::{LinearData, LinearResponse},
    },
    utils::{misc::convert_os_to_str, print_table::print_linear_results},
};

pub struct Commands<'a> {
    args: &'a Args,
    main: String,
    rest: Vec<String>,
    network_client: &'a NetworkClient,
}

impl<'a> Commands<'a> {
    pub fn new(args: &'a Args, pargs: Arguments, network_client: &'a NetworkClient) -> Self {
        Self {
            args,
            network_client,
            main: args.arg_inputs.to_owned(),
            rest: pargs.finish().iter().map(convert_os_to_str).collect(),
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
                let get_issue_response = self.network_client.get_issue(issue_key);
                match get_issue_response {
                    Ok(response_body) => {
                        let parsed: LinearResponse<GetIssueResponse> =
                            serde_json::from_str(&response_body).expect("Can't parse JSON");
                        if let Some(issue) = parsed.data {
                            print_linear_results(&LinearData::GetIssueIssue(issue));
                        }
                        if let Some(issue_errors) = parsed.errors {
                            println!("{:?}", issue_errors);
                        }
                        AnyOk(())
                    }
                    Err(error) => Err(any_macro!("{}", error)),
                }
            }
            Err(error) => Err(any_macro!("{}", error)),
        }
    }

    fn post_comment(&self) -> Result<(), AnyError> {
        match self.get_issue_key() {
            Ok(issue_key) => {
                let comment_body = self
                    .rest
                    .get(1..)
                    .ok_or_else(|| any_macro!("{}", MISSING_COMMENT_BODY))?
                    .join(" ");
                let post_comment_response = self.network_client.post_comment(
                    issue_key,
                    &comment_body,
                    &self.args.flag_dont_subscribe,
                );
                match post_comment_response {
                    Ok(response_body) => {
                        let parsed: LinearResponse<PostCommentResponse> =
                            serde_json::from_str(&response_body).expect("Can't parse JSON");
                        if let Some(comment_response) = parsed.data {
                            print_linear_results(&LinearData::PostCommentCommentCreate(
                                comment_response,
                            ));
                        }
                        if let Some(comment_errors) = parsed.errors {
                            println!("{:?}", comment_errors);
                        }
                        AnyOk(())
                    }
                    Err(error) => Err(any_macro!("{}", error)),
                }
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
