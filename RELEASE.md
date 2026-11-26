# NovaKit 发布指南

## v0.1.0 发布检查单

### 准备阶段

- [x] 代码仓库初始化并提交
- [x] Rust + Tauri 2 工程搭建完成
- [x] `cargo xtask ci` 门禁脚本实现（4项检查）
- [x] BASELINE.md 记录环境状态
- [x] 实现 6 + 10 + 8 + 9 = 33 工具模块单测（全部绿）

### 编译与打包

- [ ] `cargo tauri build --release` 通过
- [ ] macOS dmg 包生成成功
- [ ] 签名/不签名 dmg 可正常安装

### 验收测试

- [ ] 冷启动 < 3 秒
- [ ] 命令面板所有入口可达
- [ ] 异常处理完整（空态/忙态/错态）
- [ ] 文件操作路径穿越防护生效
- [ ] 外部组件缺失时给出安装指引

### 文档

- [x] README.md 更新
- [x] CHANGELOG.md 创建
- [ ] LICENSE 文件补全
- [ ] NOTICES 补全

### 发布流程

1. 确认所有检查项完成
2. 构建 release 版本
3. 上传 dmg 到 GitHub Releases
4. 更新 README 中的版本信息

## 构建命令

```bash
# Debug 版本（当前）
source ~/.zshrc && cargo tauri build --debug

# Release 版本（待测试）
source ~/.zshrc && cargo tauri build --release
```

## 已知限制

- 当前为骨架版本，尚未集成完整 36 个工具界面
- 需要手动安装外部组件（ffmpeg/7z）才能使用完整功能
