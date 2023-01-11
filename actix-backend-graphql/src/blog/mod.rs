mod model;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
pub use model::QueryRoot;
pub type BlogSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

