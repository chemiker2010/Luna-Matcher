//! Shared XLOPER12 parsing helpers for Luna Matcher XLL crates.

#[cfg(not(windows))]
compile_error!("luna_xll is Windows-only (Excel XLL add-ins require Windows/MSVC).");

mod excel_args;

pub use excel_args::{read_target_f64, read_values_f64, read_values_range, xloper_type_name, F64Range};
