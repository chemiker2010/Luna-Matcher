use super::engine::find_subset_sum;

const NO_MATCH_MSG: &str = "⚠️ 未找到匹配组合";

/// 旧版 Excel 单格文本摘要：列出参与凑数的项及金额。
pub fn build_legacy_summary(values: &[f64], target: f64) -> String {
    if values.is_empty() {
        return NO_MATCH_MSG.to_string();
    }

    let solutions = find_subset_sum(values, target, 1);
    let Some(indices) = solutions.iter().find(|s| !s.is_empty()) else {
        return NO_MATCH_MSG.to_string();
    };

    let parts: Vec<String> = indices
        .iter()
        .map(|&idx| format!("第{}项({}元)", idx + 1, format_amount(values[idx])))
        .collect();

    format!("🎯 凑数成功: {}", parts.join(" + "))
}

fn format_amount(value: f64) -> String {
    if value.fract().abs() < 1e-9 {
        format!("{:.0}", value)
    } else {
        format!("{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_lists_matching_items() {
        let values = [1000.0, 500.0, 7500.0, 2000.0];
        let summary = build_legacy_summary(&values, 8500.0);
        assert!(summary.contains("🎯 凑数成功"));
        assert!(summary.contains("第1项(1000元)"));
        assert!(summary.contains("第3项(7500元)"));
    }

    #[test]
    fn summary_when_no_match() {
        let values = [100.0, 200.0];
        assert_eq!(build_legacy_summary(&values, 999.0), NO_MATCH_MSG);
    }

    #[test]
    fn summary_empty_input() {
        assert_eq!(build_legacy_summary(&[], 100.0), NO_MATCH_MSG);
    }
}
