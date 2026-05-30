//! Luna Matcher Modern XLL — Excel 365 / 2021+ 动态数组版本。

#[cfg(windows)]
mod register;
#[cfg(windows)]
mod udf;

#[cfg(windows)]
pub use register::{xl_auto_close, xl_auto_open};
#[cfg(windows)]
pub use udf::{luna_debug, luna_find, luna_match_spill};

#[cfg(not(windows))]
pub fn platform_note() -> &'static str {
    "luna_modern 需在 Windows 上编译: cargo build -p luna_modern --release --target x86_64-pc-windows-msvc"
}
