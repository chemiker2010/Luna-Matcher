use super::engine::find_subset_sum;

/// 命中行的默认标记文本（避免 emoji，Excel 各版本均可显示）。
pub const MATCH_HIT_LABEL: &str = "[组合一]";

fn first_solution_rows(values: &[f64], target: f64) -> std::collections::HashSet<usize> {
    if values.is_empty() {
        return std::collections::HashSet::new();
    }

    let solutions = find_subset_sum(values, target, 1);
    solutions
        .iter()
        .find(|s| !s.is_empty())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .collect()
}

/// 为动态数组 spill 生成与输入等长的标记列。
///
/// 命中行返回 [`MATCH_HIT_LABEL`]，未命中与无解时均返回空字符串。
pub fn build_match_spill_labels(values: &[f64], target: f64) -> Vec<String> {
    if values.is_empty() {
        return Vec::new();
    }

    let matched = first_solution_rows(values, target);

    (0..values.len())
        .map(|i| {
            if matched.contains(&i) {
                MATCH_HIT_LABEL.to_string()
            } else {
                String::new()
            }
        })
        .collect()
}

/// 为 `LUNA.FIND` 生成与输入等长的序号列。
///
/// 命中行返回选区内的 1-based 序号（与金额行对齐），未命中与无解时返回空字符串。
pub fn build_find_row_labels(values: &[f64], target: f64) -> Vec<String> {
    if values.is_empty() {
        return Vec::new();
    }

    let matched = first_solution_rows(values, target);

    (0..values.len())
        .map(|i| {
            if matched.contains(&i) {
                (i + 1).to_string()
            } else {
                String::new()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spill_marks_matching_rows_only() {
        let values = [1000.0, 500.0, 7500.0, 2000.0];
        let labels = build_match_spill_labels(&values, 8500.0);
        assert_eq!(labels.len(), 4);
        assert_eq!(labels[0], MATCH_HIT_LABEL);
        assert_eq!(labels[1], "");
        assert_eq!(labels[2], MATCH_HIT_LABEL);
        assert_eq!(labels[3], "");
    }

    #[test]
    fn spill_marks_first_row_when_a1_in_solution() {
        let values = [1000.0, 500.0, 7500.0];
        let labels = build_match_spill_labels(&values, 8500.0);
        assert_eq!(labels[0], MATCH_HIT_LABEL);
        assert_eq!(labels[2], MATCH_HIT_LABEL);
        assert!(labels[1].is_empty());
    }

    #[test]
    fn find_row_labels_align_with_amount_rows() {
        // A2:A5 → 1000, 500, 7500, 2000；目标 10500 = 第1+3+4项
        let values = [1000.0, 500.0, 7500.0, 2000.0];
        let labels = build_find_row_labels(&values, 10500.0);
        assert_eq!(labels, vec!["1", "", "3", "4"]);
    }

    #[test]
    fn find_row_labels_with_leading_blank_row() {
        // A1 空，A2:A5 有数；10500 = A2+A4+A5 → 序号 2,4,5
        let values = [0.0, 1000.0, 500.0, 7500.0, 2000.0];
        let labels = build_find_row_labels(&values, 10500.0);
        assert_eq!(labels, vec!["", "2", "", "4", "5"]);
    }

    #[test]
    fn find_all_blank_when_no_solution() {
        let values = [100.0, 200.0];
        let labels = build_find_row_labels(&values, 999.0);
        assert_eq!(labels, vec!["", ""]);
    }

    #[test]
    fn spill_all_blank_when_no_solution() {
        let values = [100.0, 200.0];
        let labels = build_match_spill_labels(&values, 999.0);
        assert_eq!(labels, vec!["", ""]);
    }

    #[test]
    fn spill_empty_input() {
        assert!(build_match_spill_labels(&[], 100.0).is_empty());
    }

    #[test]
    fn spill_label_is_excel_safe_text() {
        assert!(!MATCH_HIT_LABEL.contains('\u{1F7E9}'));
        assert!(MATCH_HIT_LABEL.contains("组合一"));
    }
}
