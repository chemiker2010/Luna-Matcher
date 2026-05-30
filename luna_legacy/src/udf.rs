use luna_core::build_legacy_summary;
use luna_xll::{read_target_f64, read_values_f64};
use xll_rs::returning::XlReturn;
use xll_rs::types::*;

/// `=LUNA_MATCH_OLD(A2:A20, C1)` — 单格返回凑数结果摘要文本。
#[no_mangle]
pub unsafe extern "system" fn luna_match_old(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> *mut XLOPER12 {
    match luna_match_old_impl(px_values, px_target) {
        Ok(ptr) => ptr,
        Err(ret) => ret.into_raw(),
    }
}

fn luna_match_old_impl(
    px_values: *mut XLOPER12,
    px_target: *mut XLOPER12,
) -> Result<*mut XLOPER12, XlReturn> {
    let nums = read_values_f64(px_values).map_err(XlReturn::err)?;

    if nums.is_empty() {
        return Err(XlReturn::err(XLERR_VALUE));
    }

    let target_val = read_target_f64(px_target).map_err(XlReturn::err)?;

    let summary = build_legacy_summary(&nums, target_val);
    Ok(XlReturn::str(&summary).into_raw())
}
