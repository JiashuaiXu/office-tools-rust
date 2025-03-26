use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub parameters: Option<Vec<ScriptParameter>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptParameter {
    pub name: String,
    pub description: Option<String>,
    pub parameter_type: String,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptExecution {
    pub task_id: String,
    pub script_name: String,
    pub status: ScriptStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptStatus {
    Pending,
    Running,
    Completed,
    Failed,
} 