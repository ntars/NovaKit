[简体中文](README.md) | [English](README_EN.md)

# NovaKit

**仓库描述**：`装完系统只装这一套：离线、免登录的桌面小工具集合，Rust + Tauri 2 构建 | One kit, day one: an offline, account-free desktop utility suite built with Rust and Tauri 2.`

**话题标签**：`rust` `tauri` `desktop-app` `utilities` `offline` `privacy` `productivity` `toolkit` `macos` `cross-platform` `no-account` `local-first` `file-tools` `image-tools` `developer-tools` `clipboard` `screenshot` `zero-telemetry` `open-source` `apache-license`

![Status](https://img.shields.io/badge/status-骨架阶段-yellow?style=flat-square)
![Rust](https://img.shields.io/badge/rust-2024%20edition-b7410e?style=flat-square&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-2-24c8db?style=flat-square&logo=tauri&logoColor=white)
[![License](https://img.shields.io/badge/license-Apache--2.0%20%2F%20MIT-blue?style=flat-square)](./LICENSE)

一台新电脑装完系统后最先该装的那套桌面工具：离线、免登录、不联网。

[快速开始](#快速开始) · [核心特性](#核心特性) · [目录结构](#目录结构) · [已知问题与限制](#已知问题与限制) · [后续路线图](#后续路线图) · [许可证](#许可证)

---

## 项目简介

换一台新电脑，为了「解压、看图、截图、改配置、清启动项」，常常要装十个各自带广告、带账号、带云同步的软件。NovaKit 要解决的就是这件事：**用一套离线小工具覆盖新机第一周的高频操作**，不引入账号、云同步与遥测。

**核心思路**：以「装完系统第一周会用到什么」反推工具清单，把 36 个工具分成四层——纯逻辑、系统能力、外部组件、进程插件——每一层对应不同的实现与验收方式，而不是把它们做成一个不断膨胀的「全能软件」。

> 当前仓库处于**骨架阶段**：Tauri 2 工程与工具清单已确定，业务代码尚在搭建。README 描述的是目标形态与已落地的工程约束，未实现的能力在下文明确标注。

**适用场景**

- 刚换电脑，想要一套不联网、不注册就能用的文件、图片与开发小工具。
- 希望工具数据只留在本机，不接受账号体系与云同步。
- 想基于 Tauri 2 做一套可扩展、可审计的桌面工具箱。

**不适用场景**

- 需要 Office 级文档编辑：本项目不内嵌 Word/Excel/PPT 渲染，双击仍交给系统默认程序。
- 需要「清理大师」：本项目不删系统垃圾、不改系统设置、不写注册表。
- 需要移动端或 Web 版：当前只做桌面端。

### 优势与局限

**优势**

| 优势 | 说明 | 验证方式 |
| --- | --- | --- |
| 离线优先 | 无账号、无云同步、无遥测 | 断网后全部工具可用 |
| 分层验收 | 四层工具各有对应验证方式 | 每层单测 / 真机 / 拔组件 / 坏消息 |
| 许可分层清晰 | 代码 Apache-2.0，插件协议与文档 MIT | 见 `LICENSING.md` |

**局限**

| 局限 | 影响范围 | 临时规避方式 |
| --- | --- | --- |
| 仍处骨架阶段 | 尚无可用工具 | 关注路线图，暂不作为日常工具 |
| 依赖外部组件 | 压缩、转码类工具需 7z / ffmpeg | 启动时检测并给出安装指引 |
| 平台未全覆盖 | 首发聚焦 macOS | Windows / Linux 后续跟进 |

## 核心功能

目标形态下的 36 个工具，按四层组织。以下为规划清单，**均未在骨架阶段实现**：

- **文件类**：批量重命名、重复文件查找、哈希校验、文件分割合并、磁盘空间分析、文件名搜索、编码修复、解压压缩、压缩包预览。
- **图片类**：图片查看、格式转换、图片压缩、批量处理、屏幕取色、截图标注。
- **文本与开发类**：纯文本编辑器、Markdown 预览、文本工具箱、JSON 格式化、编码转换、正则测试、时间戳转换、二维码、颜色转换、简易 API 调试、计算器。
- **系统类**：剪贴板历史、便签待办、系统信息与启动项、进程管理、媒体播放与转码。

## 核心特性

### 离线且免登录

- **价值**：工具不应该为了用一个压缩包而要求注册账号。
- **实现方式**：不引入账号、云同步与遥测；配置写入本地 `settings.json`，数据写入本地 `novakit.db`。
- **差异化**：与带云服务的全家桶相比，本项目的网络请求面为零。
- **证据**：设计约束见 `NovaKit-开发手册.md` 第 5、8 章；骨架期 `tauri.conf.json` 不配置远端权限。

### 四层工具模型

- **价值**：不同工具的安全边界与验收方式不同，分层避免用同一种测试掩盖风险。
- **实现方式**：T0 纯 Rust、T1 系统能力、T2 外部组件、T3 进程插件；每层有独立的失败模式与验收动作。
- **差异化**：插件走 JSON-RPC 2.0 over stdio，默认全拒权限。
- **证据**：分层定义见手册 2.1；插件协议见第 4 章。

### 可审计的工程约束

- **价值**：桌面工具最容易在权限与依赖上失控，约束必须可机检。
- **实现方式**：`Cargo.toml` 与 `package.json` 锁定依赖；不引入 GPL/AGPL/SSPL；源文件头带 SPDX 标识。
- **差异化**：许可按目录分层——代码 Apache-2.0，文档与契约 MIT。
- **证据**：`LICENSING.md`、`NOTICE`；后续由 `check-licenses` 扫描。

## 设计细节与工程取舍

### 为什么按「装完系统第一周」反推

- **场景**：按功能分类容易堆成全能软件，偏离「新机第一天」。
- **处理方式**：先列真实第一周会做的动作，再映射到具体工具。
- **原因**：保证每个工具都有明确使用场景，避免为凑数而做。

### 为什么不内嵌 Office 查看

- **场景**：用户可能想直接打开 docx/xlsx。
- **处理方式**：交给系统默认程序，不在应用内渲染。
- **原因**：内嵌渲染成本高、保真难，且会拖慢安装包与启动。

### 为什么插件默认全拒

- **场景**：第三方扩展可能读取文件或发起网络请求。
- **处理方式**：manifest 声明权限，默认全拒，安装时显式授权。
- **原因**：把信任边界交给用户，而不是默认放行。

## 技术栈与依赖清单

| 类别 | 技术或组件 | 版本约束 | 用途 |
| --- | --- | --- | --- |
| 运行时 | Rust | edition 2024 | 核心逻辑与系统能力 |
| 桌面外壳 | Tauri | 2.x | 窗口、打包与 IPC |
| 前端 | TypeScript + React | 规划中 | 界面渲染，仅经 `invoke()` 调用 |
| 存储 | SQLite / `settings.json` | 本地 | 配置与本地数据 |
| 外部组件（可选） | 7-Zip / ffmpeg | 按需 | 压缩与音视频转码 |

第三方依赖明细与许可证见 [NOTICE](./NOTICE)。

## 安装

环境要求：Rust（edition 2024 工具链）、Node.js 18+、Tauri 2 系统依赖（macOS 需 Xcode Command Line Tools）。

```bash
git clone https://github.com/ntars/NovaKit.git
cd NovaKit
cargo build
```

验证：

```bash
cargo run
```

预期输出：

```text
Hello, world!
```

当前为骨架工程，尚未提供安装包。升级：`git pull`。卸载：删除目录。

## 快速开始

最小可运行示例（当前骨架）：

```bash
cd NovaKit
cargo run
```

预期输出：

```text
Hello, world!
```

接入 Tauri 开发模式（需先补齐前端依赖）：

```bash
npm install
npm run tauri dev
```

## 配置说明

规划中的配置源为本地 `settings.json`，骨架阶段尚无配置项。

| 键名 | 类型 | 默认值 | 必填 | 取值范围 | 作用 | 示例 |
| --- | --- | --- | --- | --- | --- | --- |
| `theme` | string | `system` | 否 | `system` / `light` / `dark` | 界面主题 | `dark` |
| `language` | string | `zh-CN` | 否 | `zh-CN` / `en-US` | 界面语言 | `zh-CN` |
| `tools.external.ffmpeg` | string | 自动检测 | 否 | 可执行文件路径 | 指定 ffmpeg 路径 | `/usr/local/bin/ffmpeg` |

配置优先级：`settings.json` > 内置默认值。热更新：部分界面项即时生效，其余需重启。

## API 或模块说明

骨架阶段仅暴露 Tauri 命令层占位，尚无稳定 API。

```rust
#[tauri::command]
fn ping() -> String
```

参数：无。返回值：固定字符串。错误码：暂无。

调用示例：

```ts
import { invoke } from '@tauri-apps/api/core';
const pong = await invoke<string>('ping');
```

限流与配额：无。

## 目录结构

```text
NovaKit/
├── Cargo.toml           # Rust 包定义
├── package.json         # 前端脚本与依赖
├── src/                 # Rust 入口
├── src-tauri/           # Tauri 配置、能力声明与图标
│   ├── tauri.conf.json  # 窗口、打包与安全配置
│   ├── capabilities/    # 权限声明
│   └── src/             # 桌面端命令层
└── NovaKit-开发手册.md   # 产品、架构与验收手册
```

## 性能表现

骨架阶段无性能数据。设计目标（非实测）：

| 指标 | 目标 | 说明 |
| --- | --- | --- |
| 冷启动 | 3 秒内 | 工具按需加载，不预载全部 |
| 安装包 | 单平台 100 MB 内 | 外部组件（7z/ffmpeg）按需分发 |

## 安全与隐私

数据收集与存储：不收集用户数据；配置与本地数据仅存本机。

传输加密：默认不发起网络请求；外部组件调用不拼接 shell 字符串。

权限模型：插件 manifest 声明能力，默认全拒，安装时显式授权。

已知风险：加载第三方插件会引入其自身风险；只从可信来源安装。

漏洞上报渠道：<ntars@qq.com>。

## 兼容性说明

| 维度 | 支持范围 | 验证状态 |
| --- | --- | --- |
| 操作系统 | macOS（首发）；Windows / Linux 计划中 | macOS 骨架可编译 |
| 运行时 | Rust edition 2024 | 已验证 |
| 桌面外壳 | Tauri 2 | 已验证 |
| 数据库 | 本地 SQLite | 规划中 |

明确不支持：移动端、Web 版、32 位系统。

## 已知问题与限制

**已知问题**

| 问题描述 | 影响范围 | 临时规避方案 | 跟踪链接 |
| --- | --- | --- | --- |
| 仅有脚手架，无可用工具 | 全部功能 | 等待路线图落地 | [NovaKit-开发手册.md](NovaKit-开发手册.md) |

**当前限制**：业务代码未开始；前端依赖尚未接入；无安装包。

**未实现功能**

| 功能 | 未实现原因 | 计划状态 |
| --- | --- | --- |
| 36 个工具 | 处于骨架阶段 | 按 v0.1 排期 |
| 进程插件协议 | 待核心工具稳定 | 0.1 只定义不开放 |
| 自动更新 | 未进入发布流程 | 排期中 |

## 更新日志

版本变更遵循语义化版本与 Keep a Changelog 结构。当前 `0.1.0` 为骨架版本，尚无 `CHANGELOG.md`。

## 后续路线图

> 下列内容为计划项，非交付承诺；实际范围以发布记录为准。

| 计划项 | 状态 | 目标版本 |
| --- | --- | --- |
| 核心工具落地（文件、图片、文本） | 计划中 | 0.1.0 |
| 系统能力工具（剪贴板、截图、进程） | 计划中 | 0.1.0 |
| 外部组件检测与调用 | 计划中 | 0.1.0 |
| 进程插件协议 | 待核心稳定 | 未定 |

## 参与贡献

- 开发环境搭建：安装 Rust 2024 工具链、Node.js 18+、Tauri 2 依赖。
- 分支模型：基于 `main` 切特性分支。
- 提交规范：Conventional Commits，中文描述改动本身。
- PR 要求：改动单一、`cargo build` 与 `cargo test` 通过。
- 行为准则：不引入 GPL/AGPL/SSPL 依赖；新增源文件须带 SPDX 标识。

## 许可证

Copyright © 2026 星玖工作室 Ntarstudio. Licensed under Apache-2.0 / MIT.

本项目按目录分层许可：代码采用 Apache-2.0，文档、契约与插件协议采用 MIT。全文见 [LICENSE](./LICENSE) 与 [LICENSE-MIT](./LICENSE-MIT)，分层说明见 [LICENSING.md](./LICENSING.md)。
