use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]  // 暂时禁用未使用代码警告
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Script not found: {0}")]
    ScriptNotFound(String),

    #[error("Script execution failed: {0}")]
    ScriptExecutionFailed(String),

    #[error("Invalid script parameters: {0}")]
    InvalidParameters(String),

    #[error("Task not found: {0}")]
    TaskNotFound(String),
}

pub type AppResult<T> = Result<T, AppError>; 