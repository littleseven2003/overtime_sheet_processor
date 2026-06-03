use std::sync::mpsc;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::excel::reader;
use crate::models::FileMeta;

/// 打开文件选择对话框（异步，不阻塞主线程）
#[tauri::command]
pub async fn open_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .add_filter("Excel 文件", &["xlsx", "xls"])
        .set_title("选择加班数据源文件")
        .pick_file(move |file_path| {
            let _ = tx.send(file_path);
        });

    // 在新线程中等待用户选择文件，避免阻塞主线程
    let path = tokio::task::spawn_blocking(move || {
        rx.recv().ok()
    })
    .await
    .map_err(|e| format!("等待文件选择失败: {}", e))?
    .ok_or("未收到文件选择结果")?;

    match path {
        Some(fp) => {
            let path = fp.into_path().map_err(|e| format!("获取文件路径失败: {}", e))?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}

/// 打开保存文件对话框（异步，不阻塞主线程）
#[tauri::command]
pub async fn save_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = mpsc::channel();

    app.dialog()
        .file()
        .add_filter("Excel 文件", &["xlsx"])
        .set_title("保存结果文件")
        .set_file_name("result.xlsx")
        .save_file(move |file_path| {
            let _ = tx.send(file_path);
        });

    let path = tokio::task::spawn_blocking(move || {
        rx.recv().ok()
    })
    .await
    .map_err(|e| format!("等待文件保存失败: {}", e))?
    .ok_or("未收到文件保存结果")?;

    match path {
        Some(fp) => {
            let path = fp.into_path().map_err(|e| format!("获取文件路径失败: {}", e))?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}

/// 读取 Excel 文件元数据
#[tauri::command]
pub fn read_file_meta(file_path: String) -> Result<FileMeta, String> {
    reader::read_excel_meta(&file_path)
}
