//! Luna 凑数核心算法层（平台无关，无 Excel 依赖）。

mod amount_grid;
mod engine;
mod item;
mod legacy_summary;
mod spill;

pub use amount_grid::extract_amounts;

pub use engine::{find_subset_sum, MatchEngine, DEFAULT_MAX_SOLUTIONS};
pub use item::{target_to_cents, FinancialItem};
pub use legacy_summary::build_legacy_summary;
pub use spill::{build_find_row_labels, build_match_spill_labels, MATCH_HIT_LABEL};
