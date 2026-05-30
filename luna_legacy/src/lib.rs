//! Luna Matcher Legacy XLL — 旧版 Excel 单格文本摘要。

#[cfg(windows)]
mod register;
#[cfg(windows)]
mod udf;

#[cfg(windows)]
pub use register::{xl_auto_close, xl_auto_open};
#[cfg(windows)]
pub use udf::luna_match_old;

#[cfg(not(windows))]
pub fn platform_note() -> &'static str {
    "luna_legacy 需在 Windows 上编译: cargo build -p luna_legacy --release --target x86_64-pc-windows-msvc"
}
