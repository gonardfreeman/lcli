use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/config/linear.graphql",
    query_path = "src/config/get_query.graphql",
    response_derives = "Debug"
)]
pub struct GetIssue;
