# cnctd_bump

CLI and library for bumping version numbers in Cargo.toml and package.json.

## Installation

```bash
cargo install cnctd_bump
```

## CLI Usage

Navigate to your project directory and run:

```bash
cnctd-bump patch    # 0.1.0 -> 0.1.1
cnctd-bump minor    # 0.1.0 -> 0.2.0
cnctd-bump major    # 0.1.0 -> 1.0.0
```

If no version part is specified, defaults to "patch".

## Library Usage

```toml
[dependencies]
cnctd_bump = { git = "https://github.com/Connected-Dot/cnctd_crates.git" }
```

```rust
use cnctd_bump::bump_version;

bump_version("patch")?;
```

## Documentation

See [CLAUDE.md](./CLAUDE.md) for detailed documentation.
