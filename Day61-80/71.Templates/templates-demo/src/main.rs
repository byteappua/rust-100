use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    user: String,
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

struct Item {
    id: u32,
    name: String,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    items: Vec<Item>,
}

/// A wrapper type that we'll use to implement `IntoResponse` for `askama` templates.
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {
        user: "Guest".to_string(),
    };
    HtmlTemplate(template)
}

async fn hello_handler(Path(name): Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

async fn list_handler() -> impl IntoResponse {
    let items = vec![
        Item { id: 1, name: "Rust".to_string() },
        Item { id: 2, name: "Axum".to_string() },
        Item { id: 3, name: "Askama".to_string() },
    ];
    let template = ListTemplate { items };
    HtmlTemplate(template)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "templates_demo=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/hello/{name}", get(hello_handler))
        .route("/list", get(list_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
