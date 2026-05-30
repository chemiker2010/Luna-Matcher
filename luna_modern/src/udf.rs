use luna_core::{build_find_row_labels, build_match_spill_labels};
use luna_xll::{read_target_f64, read_values_range, xloper_type_name};
use xll_rs::convert::build_multi;
use xll_rs::returning::XlReturn;
use xll_rs::types::*;

/// `=LUNA.FIND(A1:A20, C1)` — 与金额列等长 spill，命中行显示 1-based 序号。
///
/// 公式应写在 **B1**（与 `A1` 同行）；未命中行空白，无解时整列空白。
#[no_mangle]
pub unsafe extern "system" fn luna_find(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> *mut XLOPER12 {
    match luna_find_impl(px_values, px_target) {
        Ok(ptr) => ptr,
        Err(ret) => ret.into_raw(),
    }
}

/// `=LUNA_MATCH_SPILL(A1:A20, C1)` — 在公式列向下 spill，命中行显示 `[组合一]`。
///
/// 公式应写在 **B1**（与 `A1` 同行），金额列选 **A 列**；`C1` 为目标金额。
#[no_mangle]
pub unsafe extern "system" fn luna_match_spill(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> *mut XLOPER12 {
    match luna_match_spill_impl(px_values, px_target) {
        Ok(ptr) => ptr,
        Err(ret) => ret.into_raw(),
    }
}

/// `=LUNA.DEBUG(A1:A20, C1)` — 诊断插件读到的参数（调试用）。
#[no_mangle]
pub unsafe extern "system" fn luna_debug(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> *mut XLOPER12 {
    match luna_debug_impl(px_values, px_target) {
        Ok(ptr) => ptr,
        Err(ret) => ret.into_raw(),
    }
}

fn luna_find_impl(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> Result<*mut XLOPER12, XlReturn> {
    let range = read_values_range(px_values).map_err(XlReturn::err)?;

    if range.values.is_empty() {
        return Err(XlReturn::err(XLERR_VALUE));
    }

    let target_val = read_target_f64(px_target).map_err(XlReturn::err)?;

    let labels = build_find_row_labels(&range.values, target_val);
    let cells: Vec<XLOPER12> = labels.iter().map(|label| XLOPER12::from_str(label)).collect();

    let (out_rows, out_cols) = range.spill_shape();
    debug_assert_eq!(cells.len(), out_rows * out_cols);
    if cells.len() != out_rows * out_cols {
        return Err(XlReturn::err(XLERR_VALUE));
    }

    let ptr = build_multi(cells, out_rows, out_cols);
    if ptr.is_null() {
        return Err(XlReturn::err(XLERR_VALUE));
    }
    Ok(ptr)
}

fn luna_match_spill_impl(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> Result<*mut XLOPER12, XlReturn> {
    let range = read_values_range(px_values).map_err(XlReturn::err)?;

    if range.values.is_empty() {
        return Err(XlReturn::err(XLERR_VALUE));
    }

    let target_val = read_target_f64(px_target).map_err(XlReturn::err)?;

    let labels = build_match_spill_labels(&range.values, target_val);
    let cells: Vec<XLOPER12> = labels.iter().map(|label| XLOPER12::from_str(label)).collect();

    let (out_rows, out_cols) = range.spill_shape();
    debug_assert_eq!(cells.len(), out_rows * out_cols);
    if cells.len() != out_rows * out_cols {
        return Err(XlReturn::err(XLERR_VALUE));
    }

    let ptr = build_multi(cells, out_rows, out_cols);
    if ptr.is_null() {
        return Err(XlReturn::err(XLERR_VALUE));
    }
    Ok(ptr)
}

fn luna_debug_impl(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> Result<*mut XLOPER12, XlReturn> {
    let values_type = xloper_type_name(px_values);
    let target_type = xloper_type_name(px_target);

    let range_result = read_values_range(px_values);
    let target_result = read_target_f64(px_target);

    let nums_text = match &range_result {
        Ok(range) => {
            if range.values.is_empty() {
                "[]".to_string()
            } else {
                let parts: Vec<String> = range.values.iter().map(|v| format!("{v}")).collect();
                format!(
                    "source={}x{} spill={}x{} nums=[{}]",
                    range.source_rows,
                    range.source_cols,
                    range.spill_rows,
                    range.spill_cols,
                    parts.join(", ")
                )
            }
        }
        Err(code) => format!("<err {code}>"),
    };

    let target_text = match target_result {
        Ok(v) => format!("{v}"),
        Err(code) => format!("<err {code}>"),
    };

    let msg = format!(
        "values_type={values_type} nums={nums_text} | target_type={target_type} target={target_text}"
    );
    Ok(XlReturn::str(&msg).into_raw())
}
