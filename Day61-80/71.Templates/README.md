# Day 71: 模板引擎 (Askama/Tera)

## 学习目标
- 掌握 Askama 模板引擎
- 学习模板语法
- 实现服务端渲染
- 理解模板继承

## Askama 简介

Askama 是一个类型安全的模板引擎，在编译时检查模板。

### 安装

```toml
[dependencies]
askama = "0.15"
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

## 基本使用

### 简单模板与 Axum 集成

由于 Askama 0.13+ 移除了 `askama_axum` 集成库，我们需要手动实现 `IntoResponse` 包装器：

```rust
use askama::Template;
use axum::{
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
    http::StatusCode,
};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

// 包装器结构体，用于为任意 Askama 模板实现 IntoResponse
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

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {
        name: "World".to_string(),
    };
    HtmlTemplate(template)
}
```

```html
<!-- templates/hello.html -->
<!DOCTYPE html>
<html>
<head>
    <title>Hello</title>
</head>
<body>
    <h1>Hello, {{ name }}!</h1>
</body>
</html>
```

## 模板语法

### 变量

```html
<p>{{ user.username }}</p>
<p>{{ user.email }}</p>
```

### 条件

```html
{% if user.is_admin %}
    <p>Admin Panel</p>
{% else %}
    <p>User Panel</p>
{% endif %}
```

### 循环

```html
<ul>
{% for post in posts %}
    <li>{{ post.title }}</li>
{% endfor %}
</ul>
```

### 过滤器

```html
<p>{{ text|upper }}</p>
<p>{{ text|truncate(50) }}</p>
```

## 模板继承

### 基础模板

```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}My Site{% endblock %}</title>
</head>
<body>
    <nav>
        <a href="/">Home</a>
        <a href="/about">About</a>
    </nav>
    
    <main>
        {% block content %}{% endblock %}
    </main>
    
    <footer>
        <p>&copy; 2024 My Site</p>
    </footer>
</body>
</html>
```

### 子模板

```html
<!-- templates/index.html -->
{% extends "base.html" %}

{% block title %}Home - My Site{% endblock %}

{% block content %}
    <h1>Welcome</h1>
    <p>This is the home page.</p>
{% endblock %}
```

## 完整博客示例

```rust
use askama::Template;
use axum::{
    Router,
    routing::get,
    extract::{State, Path},
    response::Html,
};

#[derive(Template)]
#[template(path = "blog/index.html")]
struct BlogIndexTemplate {
    posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "blog/post.html")]
struct BlogPostTemplate {
    post: Post,
    comments: Vec<Comment>,
}

async fn blog_index(State(state): State<AppState>) -> impl IntoResponse {
    let posts = get_all_posts(&state.db).await.unwrap();
    let template = BlogIndexTemplate { posts };
    HtmlTemplate(template)
}

async fn blog_post(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> impl IntoResponse {
    let post = get_post(&state.db, id).await.unwrap();
    let comments = get_comments(&state.db, id).await.unwrap();
    let template = BlogPostTemplate { post, comments };
    HtmlTemplate(template)
}
```

详细内容请参考 Day61-80/STAGE5_OVERVIEW.md。

## 练习

1. 创建完整的博客模板
2. 实现用户仪表板
3. 添加表单验证显示
4. 实现分页组件
5. 创建可复用的模板片段

## 下一步

Day 72 将学习 WebAssembly (WASM) 简介。
