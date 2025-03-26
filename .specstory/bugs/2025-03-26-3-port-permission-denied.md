# Bug 记录：端口访问权限被拒绝 🔒

## 问题描述

运行服务器时遇到以下错误：

```
Error: Os { code: 10013, kind: PermissionDenied, message: "An attempt was made to access a socket in a way forbidden by its access permissions." }
```

### 错误类型
- 系统错误：端口访问权限被拒绝
- 错误代码：10013

### 发生时间
- 2024-03-26 22:36

### 影响范围
- 服务器无法启动
- 阻止了开发测试

## 原因分析

1. 端口 8080 可能被其他程序占用
2. 当前用户可能没有权限绑定该端口
3. Windows 系统下某些端口可能被系统保留

## 解决方案

1. 修改服务器配置，使用其他端口（如 3000）：

```rust
// src/main.rs
#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 从环境变量获取端口，默认使用 3000
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    
    log::info!("Starting Actix Tools server at http://localhost:{}", port);

    HttpServer::new(|| {
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
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
```

2. 或者以管理员权限运行（不推荐用于开发环境）：
```powershell
# 以管理员权限运行 PowerShell
Start-Process powershell -Verb RunAs
cargo run
```

3. 检查端口占用情况：
```powershell
netstat -ano | findstr :8080
```

## 验证步骤

1. 修改端口配置
2. 重新编译项目：`cargo build`
3. 运行服务器：`cargo run`
4. 访问新端口：http://localhost:3000

## 预防措施

1. 使用环境变量配置端口
2. 添加端口检测和自动切换逻辑
3. 在文档中说明端口要求
4. 添加错误处理和友好的错误提示

## 相关文档

- [Windows 端口权限问题](https://docs.microsoft.com/en-us/windows/win32/winsock/using-so-reuseaddr-and-so-exclusiveaddruse)
- [Actix-web 服务器配置](https://actix.rs/docs/server)

## 备注

- 开发环境推荐使用非特权端口（>1024）
- 可以考虑添加端口自动发现功能
- 建议在配置文件中管理服务器设置 