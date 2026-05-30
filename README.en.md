# Luna Matcher

English|[中文](README.md)

Excel add-in (XLL) for financial invoice subset-sum matching. Given a list of positive amounts and a target total, it finds a combination that sums exactly to the target and marks the matching rows in the spreadsheet.
 
License: MIT (see [LICENSE](LICENSE))

## Features

- Dynamic array spill marks for Excel 365 / 2021+
- Single-cell summary text for older Excel versions
- Amount rows aligned with mark rows
- Empty cells treated as zero (skipped in matching, row position preserved)
- Multi-column selections use the first column only
- Core engine in Rust; unit tests run without Excel

## Functions

| Plugin | Excel | Function | Output |
| --- | --- | --- | --- |
| Modern | 365 / 2021+ | LUNA_MATCH_SPILL | [组合一] on matched rows |
| Modern | 365 / 2021+ | LUNA.FIND | 1-based index on matched rows |
| Modern | 365 / 2021+ | LUNA.DEBUG | Diagnostic text |
| Legacy | 2016 / 2019 | LUNA_MATCH_OLD | Single-cell summary |

## Quick Start

### Sheet Layout

| Column | Purpose |
| --- | --- |
| A | Amounts, e.g. A1:A20 |
| B | Formula and spill marks, e.g. B1 |
| C | Target amount in C1 |

The formula row must match the first row of the amount range:

- A1:A20 -> formula in B1
- A2:A20 -> formula in B2

### Example

```
A2: 1000
A3: 500
A4: 7500
A5: 2000
C1: 8500
B1: =LUNA_MATCH_SPILL(A1:A20, C1)
```

Expected result:

```
B2: [组合一]
B4: [组合一]
```

Other formulas:

```
=LUNA.FIND(A1:A20, C1)
=LUNA.DEBUG(A1:A20, C1)
=LUNA_MATCH_OLD(A1:A20, C1)
```

### Load the Add-in

1. Open Excel
2. File -> Options -> Add-ins
3. Manage Excel Add-ins -> Go
4. Browse and select LunaMatcher_Modern.xll or LunaMatcher_Legacy.xll
5. Fully restart Excel after updating the XLL file

## Build

Requirements:

- Rust (2021 edition)
- Windows + MSVC toolchain
- Excel bitness must match the build target (usually x64)

```
git clone <your-repo-url>
cd Luna_matcher
rustup target add x86_64-pc-windows-msvc
cargo build --release -p luna_modern --target x86_64-pc-windows-msvc
cargo build --release -p luna_legacy --target x86_64-pc-windows-msvc
```

Rename outputs:

| Build artifact | Rename to |
| --- | --- |
| luna_modern.dll | LunaMatcher_Modern.xll |
| luna_legacy.dll | LunaMatcher_Legacy.xll |

On a Windows host:

```
cargo build --release -p luna_modern -p luna_legacy
```

Run tests:

```
cargo test -p luna_core
```

## Project Layout

```
Luna_matcher/
  luna_core/     Core subset-sum engine
  luna_xll/      Excel argument parsing (Windows)
  luna_modern/   Modern XLL
  luna_legacy/   Legacy XLL
  LICENSE
  README.md
  README.en.md
```

## Notes

- Positive amounts only
- The matched sum must equal the target exactly (cent precision)
- No match -> Modern functions spill blank cells
- Use column A for amounts; if multiple columns are selected, only the first column is read

## Disclaimer

This software is provided as-is for spreadsheet assistance only. Verify results before using them for business or compliance purposes.

## License

MIT License  
Copyright (c) 2026 Hongtao Lu
