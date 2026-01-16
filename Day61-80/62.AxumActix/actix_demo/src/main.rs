use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// 应用状态
struct AppState {
    users: Mutex<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct ListQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

// 路由处理器
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/api/users")]
async fn list_users(
    data: web::Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<impl Responder> {
    let users = data.users.lock().unwrap();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(users.len());
    
    Ok(web::Json(&users[start..end]))
}

#[get("/api/users/{id}")]
async fn get_user(
    data: web::Data<AppState>,
    path: web::Path<u32>,
) -> Result<impl Responder> {
    let users = data.users.lock().unwrap();
    let id = path.into_inner();
    
    users
        .iter()
        .find(|u| u.id == id)
        .map(|u| HttpResponse::Ok().json(u))
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))
}

#[post("/api/users")]
async fn create_user(
    data: web::Data<AppState>,
    payload: web::Json<CreateUserRequest>,
) -> Result<impl Responder> {
    let mut users = data.users.lock().unwrap();
    
    let id = users.len() as u32 + 1;
    let user = User {
        id,
        name: payload.name.clone(),
        email: payload.email.clone(),
    };
    
    users.push(user.clone());
    Ok(HttpResponse::Created().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 创建应用状态
    let app_state = web::Data::new(AppState {
        users: Mutex::new(vec![User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        }]),
    });
    
    println!("🚀 Server running on http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health_check)
            .service(list_users)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
