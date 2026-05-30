/// 财务项目：记录金额在选区中的原始索引与整型分值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinancialItem {
    /// Excel 选区内的 0-based 索引。
    pub index: usize,
    /// 转换后的「分」单位整数。
    pub cents: i64,
}

impl FinancialItem {
    /// 将 f64 财务金额安全转换为分单位整数；无效值返回 `None`。
    pub fn from_f64(index: usize, value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }

        let cents = (value * 100.0).round() as i64;
        Some(FinancialItem { index, cents })
    }

    /// 从选区数值批量构建有效条目，跳过零值与无效数。
    pub fn from_values(values: &[f64]) -> Vec<Self> {
        values
            .iter()
            .enumerate()
            .filter_map(|(index, &value)| {
                Self::from_f64(index, value).filter(|item| item.cents != 0)
            })
            .collect()
    }
}

/// 将目标金额（元）转换为分。
pub fn target_to_cents(target: f64) -> Option<i64> {
    if !target.is_finite() {
        return None;
    }
    Some((target * 100.0).round() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_finite() {
        assert!(FinancialItem::from_f64(0, f64::NAN).is_none());
    }

    #[test]
    fn converts_yuan_to_cents() {
        let item = FinancialItem::from_f64(3, 10.01).unwrap();
        assert_eq!(item.index, 3);
        assert_eq!(item.cents, 1_001);
    }
}
