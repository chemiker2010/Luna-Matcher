use xll_rs::register::Reg;

/// Excel 加载 Legacy XLL 时调用。
#[no_mangle]
pub extern "system" fn xl_auto_open() -> i32 {
    let reg = Reg::new();

    if reg
        .add(
            "luna_match_old",
            "QQQ$",
            "LUNA_MATCH_OLD",
            "values,target",
            "Luna 凑数",
            "旧版Excel凑数摘要(单格文本，列出参与凑数的项)",
            &["数值列", "目标金额"],
        )
        .is_err()
    {
        return 0;
    }

    1
}

#[no_mangle]
pub extern "system" fn xl_auto_close() -> i32 {
    1
}

#[no_mangle]
pub extern "system" fn xlAutoOpen() -> i32 {
    xl_auto_open()
}

#[no_mangle]
pub extern "system" fn xlAutoClose() -> i32 {
    xl_auto_close()
}
#[allow(unused_imports)]
pub use xll_rs::memory::xlAutoFree12;
