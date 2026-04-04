use graphql_client::GraphQLQuery;

use chrono::{DateTime as ChronoDateTime, Utc};
use serde_json::Value as JSON;

type DateTime = ChronoDateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/config/linear.graphql",
    query_path = "src/config/post_comment.graphql",
    response_derives = "Debug",
    variables_derives = "Debug",
    scalars = "DateTime, JSON",
    skip_serializing_none
)]
pub struct PostComment;
