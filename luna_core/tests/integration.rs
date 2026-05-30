use luna_core::{
    build_legacy_summary, build_match_spill_labels, extract_amounts, find_subset_sum, FinancialItem,
    MatchEngine, DEFAULT_MAX_SOLUTIONS, MATCH_HIT_LABEL,
};

#[test]
fn integration_multiple_solutions_for_same_target() {
    let values = [50.0, 50.0, 50.0, 50.0];
    let solutions = find_subset_sum(&values, 100.0, DEFAULT_MAX_SOLUTIONS);
    assert!(solutions.len() >= 3);
    for solution in &solutions {
        let sum: i64 = solution
            .iter()
            .map(|&idx| (values[idx] * 100.0).round() as i64)
            .sum();
        assert_eq!(sum, 10_000);
    }
}

#[test]
fn integration_financial_item_pipeline() {
    let values = [10500.0, 3000.0, 7500.0, 2000.0, 500.0];
    let items = FinancialItem::from_values(&values);
    let solutions = MatchEngine::solve(&items, 1_050_000, 1, false);
    assert_eq!(solutions.len(), 1);
    let sum: i64 = solutions[0]
        .iter()
        .map(|&idx| items.iter().find(|i| i.index == idx).unwrap().cents)
        .sum();
    assert_eq!(sum, 1_050_000);
}

#[test]
fn integration_spill_labels() {
    let values = [1000.0, 500.0, 7500.0, 2000.0];
    let labels = build_match_spill_labels(&values, 8500.0);
    assert_eq!(labels[0], MATCH_HIT_LABEL);
    assert_eq!(labels[2], MATCH_HIT_LABEL);
    assert!(labels[1].is_empty() && labels[3].is_empty());
}

#[test]
fn integration_multi_column_grid_uses_first_column() {
    let grid = [1000.0, 99.0, 500.0, 88.0, 7500.0, 77.0, 2000.0, 66.0];
    let (values, spill_rows, spill_cols) = extract_amounts(4, 2, &grid).unwrap();
    assert_eq!(values, vec![1000.0, 500.0, 7500.0, 2000.0]);
    assert_eq!((spill_rows, spill_cols), (4, 1));

    let labels = build_match_spill_labels(&values, 8500.0);
    assert_eq!(labels[0], MATCH_HIT_LABEL);
    assert_eq!(labels[2], MATCH_HIT_LABEL);
}

#[test]
fn integration_legacy_summary() {
    let values = [1000.0, 500.0, 7500.0];
    let summary = build_legacy_summary(&values, 8500.0);
    assert!(summary.starts_with("🎯 凑数成功"));
}
