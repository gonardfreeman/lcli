use crate::models::comment::post_comment::PostCommentCommentCreate;
use crate::models::issue::get_issue::GetIssueIssue;
use prettytable::*;

enum LinearResponseData {
    GetIssueIssue(GetIssueIssue),
    PostCommentCommentCreate(PostCommentCommentCreate),
}

pub fn print_linear_results(data: &LinearResponseData) {
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    match data {
        LinearResponseData::GetIssueIssue(issue) => {
            table.set_titles(row!(b => "id", "title", "description"));
            table.add_row(row!(
                issue.id,
                issue.title,
                issue.description.clone().unwrap()
            ));
        }
        LinearResponseData::PostCommentCommentCreate(post_comment_result) => {
            table.set_titles(row!(b => "id", "issue_id", "body"));
            table.add_row(row!(
                post_comment_result.comment.id,
                post_comment_result.comment.issue_id.clone().unwrap(),
                post_comment_result.comment.body
            ));
        }
    }
    table.printstd();
}
