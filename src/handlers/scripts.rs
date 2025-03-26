use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::services::script_service;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptRequest {
    pub script_name: String,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ScriptResponse {
    pub success: bool,
    pub task_id: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[post("/scripts/run")]
pub async fn run_script(req: web::Json<ScriptRequest>) -> impl Responder {
    let task_id = Uuid::new_v4().to_string();
    
    match script_service::execute_script(&req.script_name, req.parameters.as_ref()).await {
        Ok(output) => HttpResponse::Ok().json(ScriptResponse {
            success: true,
            task_id: Some(task_id),
            output: Some(output),
            error: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ScriptResponse {
            success: false,
            task_id: None,
            output: None,
            error: Some(e.to_string()),
        }),
    }
}

#[get("/scripts/status/{task_id}")]
pub async fn get_script_status(task_id: web::Path<String>) -> impl Responder {
    // TODO: 实现获取脚本执行状态的逻辑
    HttpResponse::Ok().json(serde_json::json!({
        "task_id": task_id.to_string(),
        "status": "pending",
    }))
}

#[get("/scripts/list")]
pub async fn list_scripts() -> impl Responder {
    match script_service::list_available_scripts().await {
        Ok(scripts) => HttpResponse::Ok().json(scripts),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        })),
    }
} 