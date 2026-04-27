pub const GET_ISSUE: &str = "query GetIssue($issue_key: String!) {
    issue(id: $issue_key) {
        id
        title
        description
    }
}";

pub const POST_COMMENT: &str = "mutation PostComment($input: CommentCreateInput!) {
  commentCreate(input: $input) {
    comment {
      body
      issueId
      id
    }
    lastSyncId
    success
  }
}";
