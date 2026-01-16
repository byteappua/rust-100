use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use blog_api::{
    api::{auth, post as post_api},
    middleware as auth_middleware,
    repository::{PostRepository, UserRepository},
    AppState,
};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blog_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:blog.db?mode=rwc".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let user_repo = UserRepository::new(pool.clone());
    let post_repo = PostRepository::new(pool.clone());

    let state = AppState {
        user_repository: user_repo,
        post_repository: post_repo,
    };

    let public_routes = Router::new()
        .route("/posts", get(post_api::list))
        .route("/posts/:id", get(post_api::get_post));

    let protected_routes = Router::new()
        .route("/posts", post(post_api::create))
        .route("/posts/:id", put(post_api::update).delete(post_api::delete))
        .layer(middleware::from_fn(auth_middleware::auth));

    let api_routes = Router::new().merge(public_routes).merge(protected_routes);

    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login));

    let app = Router::new()
        .route("/", get(root))
        .nest("/api/auth", auth_routes)
        .nest("/api", api_routes)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, Blog API!"
}
