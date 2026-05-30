use luna_core::extract_amounts;
use xll_rs::convert::coerce_to_owned;
use xll_rs::memory::xlAutoFree12;
use xll_rs::types::*;

/// Numeric amount column read from Excel, with spill output shape.
#[derive(Debug, Clone, PartialEq)]
pub struct F64Range {
    /// One amount per invoice row (first column when the source range is multi-column).
    pub values: Vec<f64>,
    /// Dynamic-array spill height (equals `values.len()` for vertical layouts).
    pub spill_rows: usize,
    /// Dynamic-array spill width (`1` for vertical layouts).
    pub spill_cols: usize,
    /// Original Excel selection size (for diagnostics).
    pub source_rows: usize,
    pub source_cols: usize,
}

impl F64Range {
    pub fn spill_shape(&self) -> (usize, usize) {
        (self.spill_rows, self.spill_cols)
    }
}

fn single_cell_range(value: f64) -> F64Range {
    F64Range {
        values: vec![value],
        spill_rows: 1,
        spill_cols: 1,
        source_rows: 1,
        source_cols: 1,
    }
}

fn range_from_grid(rows: usize, cols: usize, grid: Vec<f64>) -> Result<F64Range, i32> {
    let (values, spill_rows, spill_cols) =
        extract_amounts(rows, cols, &grid).ok_or(XLERR_VALUE)?;
    Ok(F64Range {
        values,
        spill_rows,
        spill_cols,
        source_rows: rows,
        source_cols: cols,
    })
}

/// Read a column/range of amounts, preserving row order.
///
/// Empty cells (`xltypeNil` / `xltypeMissing`) become `0.0` so spill labels stay aligned.
pub fn read_values_f64(p: *const XLOPER12) -> Result<Vec<f64>, i32> {
    read_values_range(p).map(|r| r.values)
}

/// Read amounts and retain spill shape for dynamic-array output.
pub fn read_values_range(p: *const XLOPER12) -> Result<F64Range, i32> {
    if p.is_null() {
        return Err(XLERR_VALUE);
    }
    let oper = unsafe { &*p };
    match oper.base_type() {
        XLTYPE_SREF | XLTYPE_REF => {
            let owned = coerce_to_owned(oper)?;
            let result = range_from_oper(unsafe { &*owned });
            xlAutoFree12(owned);
            result
        }
        _ => range_from_oper(oper),
    }
}

/// Human-readable xltype name for debug output.
pub fn xloper_type_name(p: *const XLOPER12) -> &'static str {
    if p.is_null() {
        return "null";
    }
    match unsafe { (*p).base_type() } {
        XLTYPE_NUM => "num",
        XLTYPE_STR => "str",
        XLTYPE_BOOL => "bool",
        XLTYPE_REF => "ref",
        XLTYPE_ERR => "err",
        XLTYPE_MULTI => "multi",
        XLTYPE_MISSING => "missing",
        XLTYPE_NIL => "nil",
        XLTYPE_SREF => "sref",
        XLTYPE_INT => "int",
        _ => "other",
    }
}

fn scalar_from_oper(oper: &XLOPER12) -> Result<f64, i32> {
    match oper.base_type() {
        XLTYPE_NUM => finite(unsafe { oper.val.num }),
        XLTYPE_INT => finite(unsafe { oper.val.w } as f64),
        XLTYPE_BOOL => Ok(if unsafe { oper.val.xbool } != 0 {
            1.0
        } else {
            0.0
        }),
        XLTYPE_STR => oper
            .as_string()
            .ok_or(XLERR_VALUE)
            .and_then(|s| parse_number_str(&s).ok_or(XLERR_VALUE)),
        XLTYPE_ERR => Err(unsafe { oper.val.err }),
        XLTYPE_MISSING | XLTYPE_NIL => Err(XLERR_VALUE),
        XLTYPE_MULTI => {
            let arr = unsafe { &*std::ptr::addr_of!(oper.val.array) };
            if arr.rows <= 0 || arr.columns <= 0 {
                return Err(XLERR_VALUE);
            }
            if arr.rows != 1 || arr.columns != 1 {
                return Err(XLERR_VALUE);
            }
            elem_to_f64(unsafe { &*arr.lparray })
        }
        _ => Err(XLERR_VALUE),
    }
}

/// Read a single numeric target (元) from an XLOPER12 argument.
pub fn read_target_f64(p: *const XLOPER12) -> Result<f64, i32> {
    if p.is_null() {
        return Err(XLERR_VALUE);
    }
    let oper = unsafe { &*p };
    match oper.base_type() {
        XLTYPE_SREF | XLTYPE_REF => {
            let owned = coerce_to_owned(oper)?;
            let result = scalar_from_oper(unsafe { &*owned });
            xlAutoFree12(owned);
            result
        }
        _ => scalar_from_oper(oper),
    }
}

fn range_from_oper(oper: &XLOPER12) -> Result<F64Range, i32> {
    match oper.base_type() {
        XLTYPE_NUM => Ok(single_cell_range(finite(unsafe { oper.val.num })?)),
        XLTYPE_INT => Ok(single_cell_range(finite(unsafe { oper.val.w } as f64)?)),
        XLTYPE_STR => {
            let v = oper
                .as_string()
                .ok_or(XLERR_VALUE)
                .and_then(|s| parse_number_str(&s).ok_or(XLERR_VALUE))?;
            Ok(single_cell_range(v))
        }
        XLTYPE_ERR => Err(unsafe { oper.val.err }),
        XLTYPE_MISSING | XLTYPE_NIL => Ok(F64Range {
            values: Vec::new(),
            spill_rows: 0,
            spill_cols: 0,
            source_rows: 0,
            source_cols: 0,
        }),
        XLTYPE_MULTI => {
            let arr = unsafe { &*std::ptr::addr_of!(oper.val.array) };
            if arr.rows <= 0 || arr.columns <= 0 {
                return Ok(F64Range {
                    values: Vec::new(),
                    spill_rows: 0,
                    spill_cols: 0,
                    source_rows: 0,
                    source_cols: 0,
                });
            }
            let rows = arr.rows as usize;
            let cols = arr.columns as usize;
            let total = rows * cols;
            let mut grid = Vec::with_capacity(total);
            for i in 0..total {
                grid.push(elem_to_f64_or_zero(unsafe { &*arr.lparray.add(i) })?);
            }
            range_from_grid(rows, cols, grid)
        }
        _ => Err(XLERR_VALUE),
    }
}

fn elem_to_f64(elem: &XLOPER12) -> Result<f64, i32> {
    match elem.base_type() {
        XLTYPE_NUM => finite(unsafe { elem.val.num }),
        XLTYPE_INT => finite(unsafe { elem.val.w } as f64),
        XLTYPE_BOOL => Ok(if unsafe { elem.val.xbool } != 0 {
            1.0
        } else {
            0.0
        }),
        XLTYPE_STR => elem
            .as_string()
            .ok_or(XLERR_VALUE)
            .and_then(|s| parse_number_str(&s).ok_or(XLERR_VALUE)),
        XLTYPE_NIL | XLTYPE_MISSING => Ok(0.0),
        XLTYPE_ERR => Err(unsafe { elem.val.err }),
        _ => Err(XLERR_VALUE),
    }
}

fn elem_to_f64_or_zero(elem: &XLOPER12) -> Result<f64, i32> {
    match elem.base_type() {
        XLTYPE_NIL | XLTYPE_MISSING => Ok(0.0),
        XLTYPE_ERR => Err(unsafe { elem.val.err }),
        _ => elem_to_f64(elem),
    }
}

fn finite(v: f64) -> Result<f64, i32> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(XLERR_VALUE)
    }
}

fn parse_number_str(s: &str) -> Option<f64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }
    let normalized = trimmed.replace(',', "");
    normalized.parse::<f64>().ok().filter(|v| v.is_finite())
}
