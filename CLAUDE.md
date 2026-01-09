# CLAUDE.md - cnctd_bump

> Brief reference for the version bumping library.

## Purpose

CLI and library for bumping version numbers in Rust (Cargo.toml) and NPM (package.json) projects.

## Key Exports

```rust
pub fn bump_version(part: &str) -> Result<()>;  // "major", "minor", "patch"
```

## CLI Usage

```bash
cnctd_bump patch    # 0.1.0 -> 0.1.1
cnctd_bump minor    # 0.1.0 -> 0.2.0
cnctd_bump major    # 0.1.0 -> 1.0.0
```

## Ecosystem Role

- **Used by**: cnctd_cli (bump command)

## Version

**0.1.9**

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
