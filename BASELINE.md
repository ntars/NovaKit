# NovaKit BASELINE.md

记录开发环境与门禁状态

## 环境清单（2026-11-26）

| 项目 | 版本/状态 |
|------|----------|
| Rust | 1.100.0 (`~/.cargo/bin`) |
| Cargo | 1.100.0 |
| Tauri CLI | v2.12.6 |
| Node.js | 22.22.2 |
| npm | 10.9.7 |
| pkg-config | 3.0.11 (Homebrew) |
| macOS SDK | 27.2 (命令行工具) |
| macOS | 27.2.1 arm64 |

## 门禁状态

### `cargo xtask ci` - 尚未实现

### 当前构建产物

```
src-tauri/target/debug/bundle/dmg/novakit_0.1.0_aarch64.dmg (7.22 MiB)
```

### 编译命令

```bash
source ~/.zshrc && cargo tauri build --debug
```

## 下一步计划

- [ ] 实现 `cargo xtask ci` 门禁脚本
- [ ] M1 外壳：窗口、路由、命令面板、设置
- [ ] 实现第一个工具（文本类）
