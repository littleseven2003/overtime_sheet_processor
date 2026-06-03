use rust_xlsxwriter::{Workbook, Format, FormatAlign, FormatBorder, Color};
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
    let worksheet = workbook.add_worksheet();

    // 设置工作表名称
    worksheet
        .set_name("加班数据")
        .map_err(|e| format!("设置工作表名称失败: {}", e))?;

    // 定义样式
    let header_format = Format::new()
        .set_bold()
        .set_font_name("Microsoft YaHei")
        .set_font_size(11.0)
        .set_background_color(Color::RGB(0x9BC2E6))
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let body_format = Format::new()
        .set_font_name("SimSun")
        .set_font_size(11.0)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::VerticalCenter)
        .set_text_wrap()
        .set_border(FormatBorder::Thin);

    // 表头列名（按照目标格式）
    // 序号 | 奖惩项点 | 奖惩明细 | 建议奖励金额 | 建议处罚金额 | 涉及人员 | 开发室
    let headers = ["序号", "奖惩项点", "奖惩明细", "建议奖励金额", "建议处罚金额", "涉及人员", "开发室"];
    for (col, header) in headers.iter().enumerate() {
        worksheet
            .write_string_with_format(0, col as u16, *header, &header_format)
            .map_err(|e| format!("写入表头失败: {}", e))?;
    }

    // 设置列宽
    worksheet.set_column_width(0, 8.0).ok();   // 序号
    worksheet.set_column_width(1, 16.0).ok();  // 奖惩项点
    worksheet.set_column_width(2, 25.0).ok();  // 奖惩明细
    worksheet.set_column_width(3, 16.0).ok();  // 建议奖励金额
    worksheet.set_column_width(4, 16.0).ok();  // 建议处罚金额
    worksheet.set_column_width(5, 12.0).ok();  // 涉及人员
    worksheet.set_column_width(6, 20.0).ok();  // 开发室

    // 按研究室分组
    let mut groups: Vec<(String, Vec<&OvertimeRecord>)> = Vec::new();
    let mut current_lab: Option<String> = None;
    let mut current_group: Vec<&OvertimeRecord> = Vec::new();

    for record in records {
        if current_lab.as_ref() != Some(&record.lab) {
            if let Some(lab) = current_lab.take() {
                groups.push((lab, current_group));
                current_group = Vec::new();
            }
            current_lab = Some(record.lab.clone());
        }
        current_group.push(record);
    }
    if let Some(lab) = current_lab {
        groups.push((lab, current_group));
    }

    // 写入数据行
    let mut current_row: u32 = 1;
    let mut seq: u32 = 1;

    for (lab, group_records) in &groups {
        let lab_full = format!("{}研究室", lab);
        let group_start_row = current_row;

        for record in group_records {
            let row = current_row;

            // 序号
            worksheet
                .write_number_with_format(row, 0, seq as f64, &body_format)
                .ok();

            // 奖惩项点
            worksheet
                .write_string_with_format(row, 1, "节假日交通奖励", &body_format)
                .ok();

            // 奖惩明细（加班记录）
            worksheet
                .write_string_with_format(row, 2, &record.overtime_records, &body_format)
                .ok();

            // 建议奖励金额
            worksheet
                .write_string_with_format(row, 3, &record.total_reward, &body_format)
                .ok();

            // 建议处罚金额（空）
            worksheet
                .write_string_with_format(row, 4, "", &body_format)
                .ok();

            // 涉及人员
            worksheet
                .write_string_with_format(row, 5, &record.name, &body_format)
                .ok();

            // 开发室
            worksheet
                .write_string_with_format(row, 6, &lab_full, &body_format)
                .ok();

            // 计算行高（基于换行数）
            let newline_count = record.overtime_records.matches('\n').count() as u32;
            let row_height = 15.0 * (newline_count + 1) as f64;
            worksheet.set_row_height(row, row_height).ok();

            current_row += 1;
        }

        // 合并序号列
        if current_row - 1 > group_start_row {
            worksheet
                .merge_range(group_start_row, 0, current_row - 1, 0, &seq.to_string(), &body_format)
                .ok();
        }

        // 合并奖惩项点列
        if current_row - 1 > group_start_row {
            worksheet
                .merge_range(group_start_row, 1, current_row - 1, 1, "节假日交通奖励", &body_format)
                .ok();
        }

        // 合并开发室列
        if current_row - 1 > group_start_row {
            worksheet
                .merge_range(group_start_row, 6, current_row - 1, 6, &lab_full, &body_format)
                .ok();
        }

        seq += 1;
    }

    workbook
        .save(output_path)
        .map_err(|e| format!("保存 Excel 文件失败: {}", e))?;

    Ok(())
}
