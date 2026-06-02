use rust_xlsxwriter::{Workbook, Worksheet, Format, FormatAlign, FormatBorder, Color};
use std::path::Path;

use crate::models::OvertimeRecord;

/// 写入格式化 Excel 文件
pub fn write_excel(output_path: &str, records: &[OvertimeRecord]) -> Result<(), String> {
    let path = Path::new(output_path);

    // 确保输出目录存在
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建输出目录失败: {}", e))?;
    }

    let mut workbook = Workbook::new();
    let worksheet = workbook
        .add_worksheet("加班数据")
        .map_err(|e| format!("创建工作表失败: {}", e))?;

    // 定义样式
    let header_format = Format::new()
        .set_bold()
        .set_font_name("Microsoft YaHei")
        .set_font_size(11.0)
        .set_bg_color(Color::RGB(0x9B, 0xC2, 0xE6))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let body_format = Format::new()
        .set_font_name("SimSun")
        .set_font_size(11.0)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);

    // 写入表头
    let headers = ["序号", "奖惩项点", "开发室", "涉及人员", "周末加班记录", "建议奖励金额"];
    for (col, header) in headers.iter().enumerate() {
        worksheet
            .write_string(0, col as u16, header, &header_format)
            .map_err(|e| format!("写入表头失败: {}", e))?;
    }

    // 设置列宽
    worksheet.set_column(0, 0, 8.0).ok();   // 序号
    worksheet.set_column(1, 1, 16.0).ok();  // 奖惩项点
    worksheet.set_column(2, 2, 20.0).ok();  // 开发室
    worksheet.set_column(3, 3, 12.0).ok();  // 涉及人员
    worksheet.set_column(4, 4, 25.0).ok();  // 周末加班记录
    worksheet.set_column(5, 5, 18.0).ok();  // 建议奖励金额

    // 写入数据行
    let mut current_row: u32 = 1;
    let mut seq: u32 = 1;
    let mut prev_lab: String = String::new();
    let mut lab_start_row: u32 = 1;

    for (idx, record) in records.iter().enumerate() {
        let row = current_row;
        let lab_full = format!("{}研究室", record.lab);

        // 序号
        worksheet
            .write_number(row, 0, seq as f64, &body_format)
            .ok();

        // 奖惩项点
        worksheet
            .write_string(row, 1, "节假日交通奖励", &body_format)
            .ok();

        // 开发室
        worksheet
            .write_string(row, 2, &lab_full, &body_format)
            .ok();

        // 涉及人员
        worksheet
            .write_string(row, 3, &record.name, &body_format)
            .ok();

        // 周末加班记录
        worksheet
            .write_string(row, 4, &record.overtime_records, &body_format)
            .ok();

        // 建议奖励金额
        worksheet
            .write_string(row, 5, &record.total_reward, &body_format)
            .ok();

        // 计算行高（基于换行数）
        let newline_count = record.overtime_records.matches('\n').count() as u32;
        let row_height = 15.0 * (newline_count + 1) as f64;
        worksheet.set_row(row, row_height).ok();

        // 合并连续相同研究室的单元格
        if record.lab != prev_lab {
            if !prev_lab.is_empty() && row > lab_start_row {
                // 合并上一个研究室的范围
                worksheet.merge_range(
                    lab_start_row,
                    2,
                    row - 1,
                    2,
                    &format!("{}研究室", prev_lab),
                    &body_format,
                ).ok();
            }
            lab_start_row = row;
            prev_lab = record.lab.clone();
        }

        seq += 1;
        current_row += 1;
    }

    // 合并最后一个研究室的单元格
    if !prev_lab.is_empty() && current_row > lab_start_row {
        worksheet.merge_range(
            lab_start_row,
            2,
            current_row - 1,
            2,
            &format!("{}研究室", prev_lab),
            &body_format,
        ).ok();
    }

    // 合并序号列和奖惩项点列
    if current_row > 1 {
        worksheet.merge_range(1, 0, current_row - 1, 0, "1", &body_format).ok();
        worksheet.merge_range(
            1,
            1,
            current_row - 1,
            1,
            "节假日交通奖励",
            &body_format,
        ).ok();
    }

    workbook
        .save(output_path)
        .map_err(|e| format!("保存 Excel 文件失败: {}", e))?;

    Ok(())
}
