use calamine::{open_workbook_auto, Data, Reader};
use std::path::Path;

use crate::models::FileMeta;

/// 读取 Excel 文件，提取研究室列表和总行数
pub fn read_excel_meta(file_path: &str) -> Result<FileMeta, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("文件不存在: {}", file_path));
    }

    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut reader = open_workbook_auto(path)
        .map_err(|e| format!("读取 Excel 文件失败: {}", e))?;

    let sheet_names = reader.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err("Excel 文件中没有工作表".to_string());
    }

    let first_sheet = &sheet_names[0];
    let range = reader
        .worksheet_range(first_sheet)
        .map_err(|e| format!("工作表读取错误: {}", e))?;

    let mut labs: Vec<String> = Vec::new();
    let mut total_rows: usize = 0;

    // 找到表头行和研究室列
    let mut header_row: Option<usize> = None;
    let mut lab_col: Option<usize> = None;

    for (row_idx, row) in range.rows().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if let Data::String(s) = cell {
                if s.trim() == "研究室" {
                    header_row = Some(row_idx);
                    lab_col = Some(col_idx);
                    break;
                }
            }
        }
        if header_row.is_some() {
            break;
        }
    }

    let header_row = header_row.ok_or("未找到表头行（需要包含'研究室'列）")?;
    let lab_col = lab_col.ok_or("未找到'研究室'列")?;

    // 遍历数据行，收集研究室列表
    for row in range.rows().skip(header_row + 1) {
        if let Some(cell) = row.get(lab_col) {
            if let Data::String(lab) = cell {
                let lab = lab.trim().to_string();
                if !lab.is_empty() && !labs.contains(&lab) {
                    labs.push(lab);
                }
            }
        }
        total_rows += 1;
    }

    Ok(FileMeta {
        file_path: file_path.to_string(),
        file_name,
        labs,
        total_rows,
    })
}

/// 读取 Excel 数据行，返回原始数据
pub fn read_excel_data(
    file_path: &str,
    selected_labs: &[String],
) -> Result<Vec<(String, String, String, String)>, String> {
    let path = Path::new(file_path);
    let mut reader = open_workbook_auto(path)
        .map_err(|e| format!("读取 Excel 文件失败: {}", e))?;

    let sheet_names = reader.sheet_names().to_vec();
    let first_sheet = &sheet_names[0];
    let range = reader
        .worksheet_range(first_sheet)
        .map_err(|e| format!("工作表读取错误: {}", e))?;

    // 找表头
    let mut header_row: Option<usize> = None;
    let mut col_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for (row_idx, row) in range.rows().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if let Data::String(s) = cell {
                col_map.insert(s.trim().to_string(), col_idx);
            }
        }
        if col_map.contains_key("研究室") {
            header_row = Some(row_idx);
            break;
        }
        col_map.clear();
    }

    let header_row = header_row.ok_or("未找到表头行")?;
    let lab_col = col_map.get("研究室").ok_or("未找到'研究室'列")?;
    let name_col = col_map.get("姓名").ok_or("未找到'姓名'列")?;
    let record_col = col_map
        .get("周末加班记录")
        .ok_or("未找到'周末加班记录'列")?;
    let reward_col = col_map
        .get("周末加班奖励总额")
        .ok_or("未找到'周末加班奖励总额'列")?;

    let mut records = Vec::new();

    for row in range.rows().skip(header_row + 1) {
        let lab = get_string_cell(row, *lab_col);
        if lab.is_empty() || !selected_labs.contains(&lab) {
            continue;
        }

        let name = get_string_cell(row, *name_col);
        let overtime = get_string_cell(row, *record_col);
        let reward = get_string_cell(row, *reward_col);

        records.push((name, overtime, reward, lab));
    }

    Ok(records)
}

/// 从行中获取字符串单元格值
fn get_string_cell(row: &[Data], col: usize) -> String {
    row.get(col)
        .and_then(|cell| match cell {
            Data::String(s) => Some(s.trim().to_string()),
            Data::Float(f) => Some(f.to_string()),
            Data::Int(i) => Some(i.to_string()),
            _ => None,
        })
        .unwrap_or_default()
}
