use super::item::FinancialItem;

/// 默认最多返回的组合数量。
pub const DEFAULT_MAX_SOLUTIONS: usize = 10;

/// 核心回溯凑数引擎。
pub struct MatchEngine<'a> {
    items: &'a [FinancialItem],
    max_solutions: usize,
    solutions: Vec<Vec<usize>>,
    allow_negative: bool,
}

impl<'a> MatchEngine<'a> {
    /// 求解子集和，返回每组解对应的原始索引列表。
    ///
    /// `allow_negative` 为 `false` 时启用纯正数剪枝（发票凑数常见场景）。
    pub fn solve(
        items: &'a [FinancialItem],
        target_cents: i64,
        max_solutions: usize,
        allow_negative: bool,
    ) -> Vec<Vec<usize>> {
        if items.is_empty() || max_solutions == 0 {
            return Vec::new();
        }

        if target_cents == 0 {
            return Vec::new();
        }

        if !allow_negative {
            let total: i64 = items.iter().map(|item| item.cents).sum();
            if total < target_cents {
                return Vec::new();
            }
        }

        let mut sorted_indices: Vec<usize> = (0..items.len()).collect();
        sorted_indices.sort_by(|&a, &b| {
            items[b]
                .cents
                .abs()
                .cmp(&items[a].cents.abs())
        });

        let mut engine = MatchEngine {
            items,
            max_solutions,
            solutions: Vec::new(),
            allow_negative,
        };

        let mut current_combination = Vec::new();
        engine.backtrack(&sorted_indices, target_cents, 0, &mut current_combination);

        engine.solutions
    }

    fn backtrack(
        &mut self,
        sorted_indices: &[usize],
        remain: i64,
        start: usize,
        current: &mut Vec<usize>,
    ) {
        if remain == 0 {
            if !current.is_empty() {
                self.solutions.push(current.clone());
            }
            return;
        }

        if self.solutions.len() >= self.max_solutions {
            return;
        }

        if !self.allow_negative && remain < 0 {
            return;
        }

        for i in start..sorted_indices.len() {
            let orig_idx = sorted_indices[i];
            let item_cents = self.items[orig_idx].cents;

            if !self.allow_negative && item_cents > remain {
                continue;
            }

            current.push(self.items[orig_idx].index);
            self.backtrack(sorted_indices, remain - item_cents, i + 1, current);
            current.pop();

            if self.solutions.len() >= self.max_solutions {
                return;
            }
        }
    }
}

/// 便捷入口：从 f64 金额列表直接求解。
pub fn find_subset_sum(
    values: &[f64],
    target: f64,
    max_solutions: usize,
) -> Vec<Vec<usize>> {
    let target_cents = match super::item::target_to_cents(target) {
        Some(cents) => cents,
        None => return Vec::new(),
    };

    let items = FinancialItem::from_values(values);
    MatchEngine::solve(&items, target_cents, max_solutions, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::FinancialItem;

    fn items_from(values: &[f64]) -> Vec<FinancialItem> {
        FinancialItem::from_values(values)
    }

    fn indices_to_amounts(values: &[f64], indices: &[usize]) -> i64 {
        indices
            .iter()
            .map(|&idx| (values[idx] * 100.0).round() as i64)
            .sum()
    }

    #[test]
    fn finds_unique_solution() {
        let values = [100.0, 200.0, 300.0, 500.0];
        let solutions = find_subset_sum(&values, 400.0, 10);
        assert_eq!(solutions.len(), 1);
        assert_eq!(indices_to_amounts(&values, &solutions[0]), 40_000);
    }

    #[test]
    fn finds_descending_prune_case() {
        let values = [100.0, 30.0, 10.0];
        let solutions = find_subset_sum(&values, 40.0, 10);
        assert!(!solutions.is_empty());
        assert_eq!(indices_to_amounts(&values, &solutions[0]), 4_000);
    }

    #[test]
    fn respects_max_solutions() {
        let values = [10.0, 10.0, 10.0, 10.0];
        let solutions = find_subset_sum(&values, 20.0, 2);
        assert_eq!(solutions.len(), 2);
    }

    #[test]
    fn returns_empty_when_impossible() {
        let values = [100.0, 200.0];
        let solutions = find_subset_sum(&values, 500.0, 10);
        assert!(solutions.is_empty());
    }

    #[test]
    fn handles_cent_precision() {
        let values = [10.01, 20.02];
        let solutions = find_subset_sum(&values, 30.03, 10);
        assert_eq!(solutions.len(), 1);
        assert_eq!(indices_to_amounts(&values, &solutions[0]), 3_003);
    }

    #[test]
    fn skips_zero_values() {
        let values = [0.0, 100.0, 200.0];
        let items = items_from(&values);
        assert_eq!(items.len(), 2);
        let solutions = find_subset_sum(&values, 100.0, 10);
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn empty_items_returns_empty() {
        let solutions = find_subset_sum(&[], 100.0, 10);
        assert!(solutions.is_empty());
    }
}
