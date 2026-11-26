# NovaKit 变更日志

所有重要变更将在此记录。

项目遵循 [语义化版本](https://semver.org/lang/zh-CN/) (Semantic Versioning)。

## [0.1.0] - 2026-11-26

### 骨架版本（M0-M6 完成）

此版本为项目骨架，实现了完整的开发框架与 31 个工具模块（6 + 10 + 8 + 9），但尚未包含完整的界面集成。

#### 已实现

- **M0 定基**：Cargo workspace + Tauri 2 工程 + `cargo xtask ci` 门禁
- **M1 外壳**：窗口、路由、命令面板、设置
- **M2 文本工具**（6 测试）：
  - `format`：JSON/XML 格式化
  - `encode`：UTF-8/GBK 编码转换
  - `compare`：文件差异对比
- **M3 文件工具**（10 测试）：
  - `safe_path`：路径穿越防护
  - `list`：目录列表
  - `search`：文件内容搜索
- **M4 图像/PDF/剪贴板**（8 测试）：
  - `resize`：图像缩放
  - `compress`：图像压缩
  - `clipboard`：剪贴板操作
- **M5 媒体/系统**（9 测试）：
  - `player`：媒体播放
  - `transcode`：转码
  - `component`：组件检测
- **M6 插件**（6 测试）：
  - `host`：插件宿主
  - `protocol`：JSON-RPC 协议

#### 待实现（M7）

- 36 个工具的完整界面集成
- README 更新
- 安装包构建与签名
- CHANGELOG/RELEASE.md 生成流程

#### 技术栈

- Rust 1.100.0 + edition 2024
- Tauri 2.12.6
- Node.js 22.22.2 + npm 10.9.7

#### 平台支持

- macOS 13+（arm64）

## [0.0.0] - 2026-11-25

- 初始化项目
