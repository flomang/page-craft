mod model;

use async_graphql::{EmptySubscription, Schema};

pub use model::QueryRoot;
pub use model::MutationRoot;

pub type BlogSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

