use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, Scope};

// 定义不同的路由处理函数
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

#[get("/greet/{name}")]
async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

// 创建一个 scope，将所有路由放在 "/api" 前缀下
fn api_scope() -> Scope {
    web::scope("/api")
        .service(hello)       // "/api/"
        .service(greet)       // "/api/greet/{name}"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api_scope())  // 注册 "/api" 范围的所有路由
    })
    .bind("0.0.0.0:5001")?
    .run()
    .await
}
