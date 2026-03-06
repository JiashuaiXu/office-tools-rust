use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use actix_files as fs;
use std::io;

mod handlers;
mod models;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 从环境变量获取端口，默认使用 3000
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    log::info!("Starting Actix Tools server at http://localhost:{}", port);

    // 尝试启动服务器
    match HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(
                web::scope("/api")
                    .service(handlers::scripts::run_script)
                    .service(handlers::scripts::get_script_status)
                    .service(handlers::scripts::list_scripts)
            )
            .service(handlers::pages::index)
    })
    .bind(("0.0.0.0", port))
    {
        Ok(server) => {
            log::info!("Server started successfully");
            server.run().await
        }
        Err(e) => {
            log::error!("Failed to start server: {}", e);
            if e.kind() == io::ErrorKind::PermissionDenied {
                log::error!("Permission denied. Try using a different port (>1024) or run with administrator privileges");
                log::error!("You can set the port using the PORT environment variable");
            }
            Err(e)
        }
    }
} 
