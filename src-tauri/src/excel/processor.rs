use regex::Regex;
use std::collections::HashMap;

use crate::models::OvertimeRecord;

/// 研究室排序优先级
const LAB_ORDER: &[&str] = &["智能通信", "数据算法", "新型能源", "新型材料", "智能装备"];

/// 处理原始数据，返回格式化后的记录
pub fn process_records(
    raw_data: &[(String, String, String, String)],
    log_callback: &dyn Fn(String),
) -> Result<Vec<OvertimeRecord>, String> {
    log_callback(format!("共读取 {} 条原始数据", raw_data.len()));

    let mut records: Vec<OvertimeRecord> = Vec::new();

    for (name, overtime_raw, reward, lab) in raw_data {
        let formatted_overtime = format_overtime_records(overtime_raw);
        if formatted_overtime.is_empty() {
            continue;
        }

        records.push(OvertimeRecord {
            name: name.clone(),
            overtime_records: formatted_overtime,
            total_reward: reward.clone(),
            lab: lab.clone(),
        });
    }

    log_callback(format!("筛选后有效记录: {} 条", records.len()));

    // 按研究室排序
    let mut lab_priority: HashMap<&str, usize> = HashMap::new();
    for (i, lab) in LAB_ORDER.iter().enumerate() {
        lab_priority.insert(lab, i);
    }

    records.sort_by(|a, b| {
        let pa = lab_priority.get(a.lab.as_str()).unwrap_or(&99);
        let pb = lab_priority.get(b.lab.as_str()).unwrap_or(&99);
        pa.cmp(pb)
    });

    log_callback("数据排序完成".to_string());

    Ok(records)
}

/// 格式化加班记录字符串
/// 输入: "[1.2026-05-02 奖励100, 2.2026-05-23 奖励100, 3.2026-05-31 奖励50]"
/// 输出: "5月2日加班\n5月23日加班\n5月31日加班"
fn format_overtime_records(raw: &str) -> String {
    if raw.trim().is_empty() {
        return String::new();
    }

    // 匹配 YYYY-MM-DD 格式的日期
    let re_date = Regex::new(r"(\d{4})-(\d{1,2})-(\d{1,2})").unwrap();

    let mut dates: Vec<(u32, u32, String)> = Vec::new();

    for cap in re_date.captures_iter(raw) {
        let month: u32 = cap[2].parse().unwrap_or(0);
        let day: u32 = cap[3].parse().unwrap_or(0);
        let text = format!("{}月{}日加班", month, day);
        dates.push((month, day, text));
    }

    if dates.is_empty() {
        return raw.trim().to_string();
    }

    // 按月、日排序
    dates.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    // 去重
    dates.dedup_by_key(|d| d.2.clone());

    dates.into_iter().map(|d| d.2).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_overtime_records() {
        let input = "[1.2026-05-02 奖励100, 2.2026-05-23 奖励100, 3.2026-05-31 奖励50]";
        let result = format_overtime_records(input);
        assert_eq!(result, "5月2日加班\n5月23日加班\n5月31日加班");
    }

    #[test]
    fn test_format_single_date() {
        let result = format_overtime_records("2026-05-10 奖励50");
        assert_eq!(result, "5月10日加班");
    }

    #[test]
    fn test_format_empty() {
        let result = format_overtime_records("");
        assert_eq!(result, "");
    }
}
