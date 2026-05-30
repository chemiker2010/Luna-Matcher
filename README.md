# Luna Matcher

中文|[English](README.en.md)

Excel 发票凑数插件 (XLL)。从一组正数金额中找出相加等于目标金额的组合，并在表格中按行标记结果。
 
License: MIT (see [LICENSE](LICENSE))

## 功能

- Excel 365 / 2021+ 动态数组 spill 标记
- 旧版 Excel 单格文本摘要
- 金额列与标记列逐行对齐
- 空单元格视为 0，不参与凑数
- 多列选区自动只读第一列金额
- 核心算法为 Rust，可在无 Excel 环境下运行单元测试

## 函数

| 插件 | Excel 版本 | 函数 | 说明 |
| --- | --- | --- | --- |
| Modern | 365 / 2021+ | LUNA_MATCH_SPILL | 命中行显示 [组合一] |
| Modern | 365 / 2021+ | LUNA.FIND | 命中行显示 1-based 序号 |
| Modern | 365 / 2021+ | LUNA.DEBUG | 诊断读到的参数 |
| Legacy | 2016 / 2019 | LUNA_MATCH_OLD | 单格返回凑数摘要 |

## 快速开始

### 表格布局

| 列 | 用途 |
| --- | --- |
| A | 金额列，如 A1:A20 |
| B | 公式与标记列，如 B1 |
| C | 目标金额，如 C1 |

公式行必须与金额范围起始行一致:

- A1:A20 对应 B1
- A2:A20 对应 B2

### 示例

```
A2: 1000
A3: 500
A4: 7500
A5: 2000
C1: 8500
B1: =LUNA_MATCH_SPILL(A1:A20, C1)
```

预期结果:

```
B2: [组合一]
B4: [组合一]
```

其他公式:

```
=LUNA.FIND(A1:A20, C1)
=LUNA.DEBUG(A1:A20, C1)
=LUNA_MATCH_OLD(A1:A20, C1)
```

### 加载插件

1. 打开 Excel
2. 文件 -> 选项 -> 加载项
3. 管理 Excel 加载项 -> 转到
4. 浏览并选择 LunaMatcher_Modern.xll 或 LunaMatcher_Legacy.xll
5. 更新插件后需完全退出 Excel 再重新加载

## 编译

环境要求:

- Rust (2021 edition)
- Windows + MSVC 工具链
- Excel 与编译目标位数一致 (通常为 64 位)

```
git clone <>
cd Luna_matcher
rustup target add x86_64-pc-windows-msvc
cargo build --release -p luna_modern --target x86_64-pc-windows-msvc
cargo build --release -p luna_legacy --target x86_64-pc-windows-msvc
```

重命名产物:

| 编译产物 | 重命名为 |
| --- | --- |
| luna_modern.dll | LunaMatcher_Modern.xll |
| luna_legacy.dll | LunaMatcher_Legacy.xll |

Windows 本机可直接编译:

```
cargo build --release -p luna_modern -p luna_legacy
```

运行测试:

```
cargo test -p luna_core
```

## 项目结构

```
Luna_matcher/
  luna_core/     核心凑数算法
  luna_xll/      Excel 参数读取 (Windows)
  luna_modern/   Modern 版 XLL
  luna_legacy/   Legacy 版 XLL
  LICENSE
  README.md
  README.en.md
```

## 使用说明

- 仅支持正数金额
- 凑数结果须精确等于目标 (精确到分)
- 无解时 Modern 版返回整列空白
- 金额请选 A 列; 误选多列时只读第一列

## 免责声明

本工具按原样提供，仅供表格辅助核对。使用前请自行验证结果是否满足业务与合规要求。

## License

MIT License  
Copyright (c) 2026 Hongtao Lu
