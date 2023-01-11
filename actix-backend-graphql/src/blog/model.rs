pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn posts(&self, ctx: &Context<'_>) -> Vec<Post> {
        // Retrieve all the posts from the database
        let connection = ctx.data::<DbConnection>().get().unwrap();
        use schema::posts::dsl::*;
        let results = posts.load::<Post>(connection).expect("Error loading posts");
        results
    }

    async fn post(&self, id: i32, ctx: &Context<'_>) -> Option<Post> {
        // Retrieve a specific post by ID from the database
        let connection = ctx.data::<DbConnection>().get().unwrap();
        use schema::posts::dsl::*;
        let post = posts
            .find(id)
            .first::<Post>(connection)
            .optional()
            .expect("Error loading post");
        post
    }
}

// Define the context
struct Context {
    data: web::Data<AppState>,
    request: actix_web::web::Json<GraphQLRequest>,
}

impl juniper::Context for Context {}

#[derive(Deserialize)]
struct AuthData {
    username: String,
    password: String,
}
