use xll_rs::register::Reg;

/// Excel 加载 XLL 时调用，注册 Modern 版函数。
#[no_mangle]
pub extern "system" fn xl_auto_open() -> i32 {
    let reg = Reg::new();

    if reg
        .add(
            "luna_find",
            "QQQ$",
            "LUNA.FIND",
            "values,target",
            "Luna 凑数",
            "查找凑数组合，在公式列按行显示序号(命中行1-based序号，未命中空白)",
            &["金额列(如A1:A20)", "目标金额(如C1)"],
        )
        .is_err()
    {
        return 0;
    }

    if reg
        .add(
            "luna_match_spill",
            "QQQ$",
            "LUNA_MATCH_SPILL",
            "values,target",
            "Luna 凑数",
            "在金额列旁写入公式，向下溢出标记(命中行显示[组合一]，无解时全空白)",
            &["金额列(如A1:A20)", "目标金额(如C1)"],
        )
        .is_err()
    {
        return 0;
    }

    if reg
        .add(
            "luna_debug",
            "QQQ$",
            "LUNA.DEBUG",
            "values,target",
            "Luna 凑数",
            "诊断插件读到的参数(调试用)",
            &["数值列", "目标金额"],
        )
        .is_err()
    {
        return 0;
    }

    1
}

/// Excel 卸载 XLL 时调用。
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