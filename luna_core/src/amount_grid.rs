/// 从 Excel 行优先二维网格提取金额列表，并确定 spill 输出形状。
///
/// - **多行选区**（常见：`A1:A20` 或误选 `A1:C20`）：只取**第一列**，与 B 列逐行标记对齐。
/// - **单行多列**（如 `A1:E1`）：取整行，水平 spill。
pub fn extract_amounts(
    rows: usize,
    cols: usize,
    grid: &[f64],
) -> Option<(Vec<f64>, usize, usize)> {
    if rows == 0 || cols == 0 || grid.len() != rows * cols {
        return None;
    }

    if rows == 1 && cols > 1 {
        Some((grid.to_vec(), 1, cols))
    } else {
        let values: Vec<f64> = (0..rows).map(|r| grid[r * cols]).collect();
        Some((values, rows, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_single_column() {
        let grid = [1000.0, 500.0, 7500.0, 2000.0];
        let (values, sr, sc) = extract_amounts(4, 1, &grid).unwrap();
        assert_eq!(values, vec![1000.0, 500.0, 7500.0, 2000.0]);
        assert_eq!((sr, sc), (4, 1));
    }

    #[test]
    fn vertical_multi_column_uses_first_column_only() {
        // Row-major A,B pairs: (1000, x), (500, x), (7500, x), (2000, x)
        let grid = [1000.0, 99.0, 500.0, 88.0, 7500.0, 77.0, 2000.0, 66.0];
        let (values, sr, sc) = extract_amounts(4, 2, &grid).unwrap();
        assert_eq!(values, vec![1000.0, 500.0, 7500.0, 2000.0]);
        assert_eq!((sr, sc), (4, 1));
    }

    #[test]
    fn horizontal_single_row() {
        let grid = [100.0, 200.0, 300.0];
        let (values, sr, sc) = extract_amounts(1, 3, &grid).unwrap();
        assert_eq!(values, vec![100.0, 200.0, 300.0]);
        assert_eq!((sr, sc), (1, 3));
    }

    #[test]
    fn first_row_in_solution_aligns_to_index_zero() {
        let grid = [1000.0, 500.0, 7500.0];
        let (values, sr, sc) = extract_amounts(3, 1, &grid).unwrap();
        assert_eq!(values[0], 1000.0);
        assert_eq!((sr, sc), (3, 1));
    }
}
