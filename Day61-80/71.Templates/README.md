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
askama = "0.12"
askama_axum = "0.4"
axum = "0.7"
```

## 基本使用

### 简单模板

```rust
use askama::Template;
use axum::{response::Html, routing::get, Router};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

async fn hello() -> Html<String> {
    let template = HelloTemplate {
        name: "World".to_string(),
    };
    Html(template.render().unwrap())
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

async fn blog_index(State(state): State<AppState>) -> Html<String> {
    let posts = get_all_posts(&state.db).await.unwrap();
    let template = BlogIndexTemplate { posts };
    Html(template.render().unwrap())
}

async fn blog_post(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Html<String> {
    let post = get_post(&state.db, id).await.unwrap();
    let comments = get_comments(&state.db, id).await.unwrap();
    let template = BlogPostTemplate { post, comments };
    Html(template.render().unwrap())
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
