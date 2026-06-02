use serde::{Deserialize, Serialize};

/// 文件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    pub file_path: String,
    pub file_name: String,
    pub labs: Vec<String>,
    pub total_rows: usize,
}

/// 处理请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRequest {
    pub file_path: String,
    pub selected_labs: Vec<String>,
    pub output_path: String,
}

/// 处理结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub output_path: String,
    pub total_records: usize,
    pub message: String,
}

/// 日志事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub level: String, // "info" | "success" | "error"
    pub message: String,
}

/// 加班记录行
#[derive(Debug, Clone)]
pub struct OvertimeRecord {
    pub name: String,
    pub overtime_records: String,
    pub total_reward: String,
    pub lab: String,
}
