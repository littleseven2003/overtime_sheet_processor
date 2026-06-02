use tauri::{AppHandle, Emitter};

use crate::excel::processor;
use crate::excel::reader;
use crate::excel::writer;
use crate::models::{LogEvent, ProcessRequest, ProcessResult};

/// 处理加班数据
#[tauri::command]
pub fn process_data(app: AppHandle, request: ProcessRequest) -> Result<ProcessResult, String> {
    // 1. 发送开始日志
    app.emit(
        "log",
        LogEvent {
            level: "info".into(),
            message: "开始处理文件...".into(),
        },
    )
    .map_err(|e| e.to_string())?;

    // 2. 读取 Excel 数据
    app.emit(
        "log",
        LogEvent {
            level: "info".into(),
            message: format!("读取文件: {}", request.file_path),
        },
    )
    .map_err(|e| e.to_string())?;

    let raw_data = reader::read_excel_data(&request.file_path, &request.selected_labs)
        .map_err(|e| {
            let _ = app.emit(
                "log",
                LogEvent {
                    level: "error".into(),
                    message: format!("读取文件失败: {}", e),
                },
            );
            e
        })?;

    app.emit(
        "log",
        LogEvent {
            level: "info".into(),
            message: format!("读取到 {} 条匹配记录", raw_data.len()),
        },
    )
    .map_err(|e| e.to_string())?;

    // 3. 处理数据
    let records = processor::process_records(&raw_data, &|msg| {
        let _ = app.emit(
            "log",
            LogEvent {
                level: "info".into(),
                message: msg,
            },
        );
    })
    .map_err(|e| {
        let _ = app.emit(
            "log",
            LogEvent {
                level: "error".into(),
                message: format!("数据处理失败: {}", e),
            },
        );
        e
    })?;

    // 4. 写入 Excel
    app.emit(
        "log",
        LogEvent {
            level: "info".into(),
            message: "正在生成结果文件...".into(),
        },
    )
    .map_err(|e| e.to_string())?;

    writer::write_excel(&request.output_path, &records).map_err(|e| {
        let _ = app.emit(
            "log",
            LogEvent {
                level: "error".into(),
                message: format!("写入文件失败: {}", e),
            },
        );
        e
    })?;

    // 5. 发送完成日志
    let total = records.len();
    app.emit(
        "log",
        LogEvent {
            level: "success".into(),
            message: format!("处理完成，共 {} 条记录", total),
        },
    )
    .map_err(|e| e.to_string())?;

    Ok(ProcessResult {
        success: true,
        output_path: request.output_path,
        total_records: total,
        message: "处理完成".into(),
    })
}
