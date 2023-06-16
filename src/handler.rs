use std::sync::Arc;

use axum::{
    http::{HeaderMap, StatusCode},
    response::Html,
    Extension, Form,
};

use crate::{
    form,
    model::{self, state::AppState},
    rabbitmq::topic,
    Error, Result,
};

/// 生成激活码
fn gen_active_code(_email: &str) -> String {
    // 仅作演示！这里只返回固定字符
    // 正式开发时，应该：
    // 1. 随机字符
    // 2. 与邮箱关联，存储到 redis 中
    return format!("axum.rs");
}

/// 获取激活码
fn get_active_code(_email: &str) -> String {
    // 仅作演示！这里只返回固定字符
    // 正式开发时，应该：从 redis 中通过邮箱地址取出
    return format!("axum.rs");
}

pub async fn register(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::RegisterForm>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let cfg = &state.cfg;
    let active_code = model::user::ActiveCode {
        code: gen_active_code(&frm.email),
        email: frm.email,
        email_cfg: cfg.email.clone(),
    };

    let payload = serde_json::to_string(&active_code).map_err(Error::from)?;

    // 发送消息
    topic::send(
        &cfg.rabbitmq.dsn,
        &cfg.rabbitmq.exchange_name,
        &cfg.rabbitmq.queue_name,
        &cfg.rabbitmq.routing_key,
        &payload,
    )
    .await
    .map_err(Error::from)?;

    redirect("/active")
}

pub async fn active(Form(frm): Form<form::ActiveForm>) -> Result<Html<String>> {
    let code = get_active_code(&frm.email);
    let is_ok = frm.code == code;

    active_done_ui(is_ok)
}

pub async fn register_ui() -> Result<Html<String>> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="https://cdn.tailwindcss.com?plugins=forms"></script>
    <title>用户注册</title>
</head>
<body>
    <div class="max-w-xs mx-auto my-6">
        <h1 class="text-lg font-bold my-3">用户注册</h1>
        <form action="/register" method="post">
            <div class="grid grid-cols-1 gap-6">
                <label class="block">
                    <span class="text-gray-700">用户名</span>
                    <input type="text" name="username" class="mt-1 block w-full" placeholder="请输入你的用户名" required />
                </label>
                <label class="block">
                    <span class="text-gray-700">邮箱</span>
                    <input type="email" name="email" class="mt-1 block w-full" placeholder="请输入你的邮箱" required />
                </label>
            </div>
           
            <div class="my-6">
                <button class="border px-3 py-1 bg-blue-600 text-white text-lg">注册</button>
            </div>
        </form>
    </div>
</body>
</html>"#;

    Ok(Html(html.to_string()))
}

pub async fn active_ui() -> Result<Html<String>> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="https://cdn.tailwindcss.com?plugins=forms"></script>
    <title>激活账号</title>
</head>
<body>
    <div class="max-w-xs mx-auto my-6">
        <h1 class="text-lg font-bold my-3">激活账号</h1>
        <form action="/active" method="post">
            <div class="grid grid-cols-1 gap-6">
                
                <label class="block">
                    <span class="text-gray-700">邮箱</span>
                    <input type="email" name="email" class="mt-1 block w-full" placeholder="请输入你的邮箱" required />
                </label>
                <label class="block">
                    <span class="text-gray-700">激活码</span>
                    <input type="text" name="code" class="mt-1 block w-full" placeholder="请输入你的激活码" required />
                </label>
            </div>
           
            <div class="my-6">
                <button class="border px-3 py-1 bg-blue-600 text-white text-lg">激活</button>
            </div>
        </form>
    </div>
</body>
</html>"#;

    Ok(Html(html.to_string()))
}

fn active_done_ui(is_ok: bool) -> Result<Html<String>> {
    let text_color = if is_ok {
        "text-green-600"
    } else {
        "text-red-600"
    };
    let msg = if is_ok {
        "你的账号已成功激活"
    } else {
        "激活码错误，请检查你的邮箱"
    };
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="https://cdn.tailwindcss.com"></script>
    <title>激活账号</title>
</head>
<body>
    <div class="max-w-xs mx-auto my-6">
        <h1 class="text-lg font-bold my-3">激活账号</h1>
        <div class="my-3 {} text-xl">{}</div>
       
    </div>
</body>
</html>"#,
        text_color, msg
    );

    Ok(Html(html.to_string()))
}

fn redirect(url: &str) -> Result<(StatusCode, HeaderMap, ())> {
    let mut header = HeaderMap::new();
    header.insert(axum::http::header::LOCATION, url.parse().unwrap());

    Ok((StatusCode::FOUND, header, ()))
}
