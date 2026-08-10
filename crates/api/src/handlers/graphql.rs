use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;

use crate::AppSchema;

pub(crate) async fn handler(State(schema): State<AppSchema>, request: GraphQLRequest) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}
