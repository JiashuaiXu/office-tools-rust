use std::path::Path;
use tokio::fs;
use crate::models::error::{AppError, AppResult};
use crate::models::script::Script;
use crate::utils::{get_script_path, is_python_script};
use serde_json::Value;
use std::process::Command;

pub async fn execute_script(script_name: &str, parameters: Option<&Value>) -> AppResult<String> {
    let script_path = get_script_path(script_name);
    let path = Path::new(&script_path);
    
    if !path.exists() || !is_python_script(path) {
        return Err(AppError::ScriptNotFound(script_name.to_string()));
    }

    let mut command = Command::new("python");
    command.arg(&script_path);

    // 如果有参数，添加到命令中
    if let Some(params) = parameters {
        if let Value::Object(map) = params {
            for (key, value) in map {
                if !is_valid_parameter_name(key) {
                    return Err(AppError::InvalidParameters(format!("Invalid parameter name: {}", key)));
                }
                command.arg(format!("--{}", key));
                command.arg(value.to_string());
            }
        }
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(AppError::ScriptExecutionFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn list_available_scripts() -> AppResult<Vec<Script>> {
    let mut scripts = Vec::new();
    let mut entries = fs::read_dir("scripts").await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if is_python_script(&path) {
            if let Some(name) = path.file_stem() {
                let script_name = name.to_string_lossy().to_string();
                scripts.push(Script {
                    name: script_name.clone(),
                    description: None,
                    path: get_script_path(&script_name),
                    parameters: None,
                });
            }
        }
    }

    Ok(scripts)
}

// 验证参数名是否合法
fn is_valid_parameter_name(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
} 