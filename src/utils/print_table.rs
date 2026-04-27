use crate::models::linear::LinearData;
use prettytable::*;

pub fn print_linear_results(data: &LinearData) {
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    match data {
        LinearData::GetIssueIssue(issue) => {
            table.set_titles(row!(b => "id", "title", "description"));
            table.add_row(row!(
                issue.issue.id,
                issue.issue.title,
                issue.issue.description.clone().unwrap()
            ));
        }
        LinearData::PostCommentCommentCreate(post_comment_result) => {
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
