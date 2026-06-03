use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::excel::reader;
use crate::models::FileMeta;

/// 打开文件选择对话框
#[tauri::command]
pub fn open_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Excel 文件", &["xlsx", "xls"])
        .set_title("选择加班数据源文件")
        .blocking_pick_file();

    match file {
        Some(fp) => {
            let path = fp.into_path().map_err(|e| format!("获取文件路径失败: {}", e))?;
            Ok(Some(path.to_string_lossy().to_string()))
        }
        None => Ok(None),
    }
}

/// 打开保存文件对话框
#[tauri::command]
pub fn save_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Excel 文件", &["xlsx"])
        .set_title("保存结果文件")
        .set_file_name("result.xlsx")
        .blocking_save_file();

    match file {
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
