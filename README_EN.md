[简体中文](README.md) | [English](README_EN.md)

# NovaKit

**Description**: `装完系统，只装这一套：离线、免登录的桌面小工具集合，Rust + Tauri 2 构建 | One kit, day one: an offline, account-free desktop utility suite built with Rust and Tauri 2.`

**Topics**: `rust` `tauri` `desktop-app` `utilities` `offline` `privacy` `productivity` `toolkit` `macos` `cross-platform` `no-account` `local-first` `file-tools` `image-tools` `developer-tools` `clipboard` `screenshot` `zero-telemetry` `open-source` `apache-license`

![Status](https://img.shields.io/badge/status-scaffold-yellow?style=flat-square)
![Rust](https://img.shields.io/badge/rust-2024%20edition-b7410e?style=flat-square&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-2-24c8db?style=flat-square&logo=tauri&logoColor=white)
[![License](https://img.shields.io/badge/license-Apache--2.0%20%2F%20MIT-blue?style=flat-square)](./LICENSE)

The desktop utility kit you install first on a brand-new machine: offline, account-free, no telemetry.

[Quick Start](#quick-start) · [Features](#key-features) · [Layout](#project-layout) · [Known Issues](#known-issues-and-limitations) · [Roadmap](#roadmap) · [License](#license)

---

## Introduction

On a new computer you end up installing ten separate apps — each with ads, an account, or cloud sync — just to unzip a file, view an image, take a screenshot, edit a config, or prune startup items. NovaKit targets exactly that: **one offline kit that covers the high-frequency tasks of a machine's first week**, with no accounts, no cloud sync, and no telemetry.

**Core idea**: start from "what will I actually do in week one", then map those actions to 36 tools in four tiers — pure logic, system capability, external component, and process plugin. Each tier has its own implementation and acceptance method instead of growing into an all-in-one suite.

> This repository is at **scaffold stage**: the Tauri 2 project and the tool list are set, while business code is still being built. This README describes the target shape and the engineering constraints already in place; unimplemented capabilities are explicitly labeled below.

**When it fits**

- You just switched machines and want an offline, sign-in-free set of file, image, and developer tools.
- You want tool data to stay on your machine, with no accounts or cloud sync.
- You want a Tauri 2 base for an extensible, auditable desktop toolbox.

**When it does not fit**

- Office-grade document editing: this project does not embed Word/Excel/PPT rendering; double-click still opens the system default app.
- "Cleaner" behavior: it does not delete system junk, change system settings, or touch the registry.
- Mobile or web versions: desktop only for now.

### Strengths and Limits

**Strengths**

| Strength | Detail | Verification |
| --- | --- | --- |
| Offline-first | No accounts, cloud sync, or telemetry | Every tool works with the network off |
| Tiered acceptance | Each tier has its own verification method | Unit test / real device / unplug component / malformed message |
| Clear license layering | Apache-2.0 code, MIT docs and plugin protocol | See `LICENSING.md` |

**Limits**

| Limit | Scope | Workaround |
| --- | --- | --- |
| Still at scaffold stage | No usable tools yet | Watch the roadmap; not for daily use |
| Depends on external components | Archive and transcoding tools need 7z / ffmpeg | Detected at launch with install guidance |
| Not cross-platform yet | macOS first | Windows / Linux to follow |

## Key Features

The target 36 tools, organized in four tiers. The list below is a plan; **none of it is implemented at scaffold stage**:

- **Files**: batch rename, duplicate finder, hash check, split/merge, disk usage, filename search, filename encoding repair, archive/unarchive, archive preview.
- **Images**: viewer, format conversion, compression, batch processing, color picker, screenshot annotation.
- **Text and developer**: plain-text editor, Markdown preview, text toolbox, JSON formatter, encoding conversion, regex tester, timestamp converter, QR code, color converter, simple API client, calculator.
- **System**: clipboard history, notes/todos, system info and startup items, process manager, media playback and transcoding.

## Core Features

### Offline and account-free

- **Value**: nobody should register an account just to unzip a file.
- **Implementation**: no accounts, cloud sync, or telemetry; config goes to a local `settings.json`, data to a local `novakit.db`.
- **Difference**: compared with cloud-backed suites, the network surface is zero.
- **Evidence**: constraints in `NovaKit-开发手册.md` chapters 5 and 8; the scaffold `tauri.conf.json` grants no remote permissions.

### Four-tier tool model

- **Value**: tools have different security boundaries and acceptance methods; tiers prevent one test from masking another risk.
- **Implementation**: T0 pure Rust, T1 system capability, T2 external component, T3 process plugin, each with its own failure modes and acceptance steps.
- **Difference**: plugins speak JSON-RPC 2.0 over stdio with all permissions denied by default.
- **Evidence**: tier definitions in the manual section 2.1; plugin protocol in chapter 4.

### Auditable engineering constraints

- **Value**: desktop tools most often drift on permissions and dependencies, so the constraints must be machine-checkable.
- **Implementation**: `Cargo.toml` and `package.json` pin dependencies; no GPL/AGPL/SSPL; every source file carries an SPDX identifier.
- **Difference**: licenses are layered by directory — code Apache-2.0, docs and contracts MIT.
- **Evidence**: `LICENSING.md` and `NOTICE`; `check-licenses` will scan in later milestones.

## Design Details and Trade-offs

### Why start from "week one"

- **Scenario**: grouping by feature tends to inflate into an all-in-one suite.
- **Approach**: list the real actions of week one first, then map them to tools.
- **Reason**: every tool has a concrete use case instead of existing to fill a grid.

### Why no embedded Office viewer

- **Scenario**: a user may want to open docx/xlsx directly.
- **Approach**: hand it to the system default app; no in-app rendering.
- **Reason**: embedded rendering is costly, hard to keep faithful, and bloats the installer and startup.

### Why plugins are denied by default

- **Scenario**: third-party extensions may read files or make network calls.
- **Approach**: manifest declares permissions, everything denied by default, explicit grant at install time.
- **Reason**: the trust boundary belongs to the user.

## Tech Stack and Dependencies

| Category | Technology | Version | Purpose |
| --- | --- | --- | --- |
| Runtime | Rust | edition 2024 | Core logic and system capabilities |
| Desktop shell | Tauri | 2.x | Window, bundling, IPC |
| Frontend | TypeScript + React | Planned | UI, calling only through `invoke()` |
| Storage | SQLite / `settings.json` | Local | Config and local data |
| External (optional) | 7-Zip / ffmpeg | On demand | Archives and media transcoding |

Dependency details and licenses: see [NOTICE](./NOTICE).

## Installation

Requirements: Rust (edition 2024 toolchain), Node.js 18+, and Tauri 2 system dependencies (Xcode Command Line Tools on macOS).

```bash
git clone https://github.com/ntars/NovaKit.git
cd NovaKit
cargo build
```

Verify:

```bash
cargo run
```

Expected output:

```text
Hello, world!
```

No installer yet; this is a scaffold. Upgrade: `git pull`. Uninstall: delete the directory.

## Quick Start

Minimal runnable example (current scaffold):

```bash
cd NovaKit
cargo run
```

Expected output:

```text
Hello, world!
```

Start Tauri dev mode (frontend dependencies required):

```bash
npm install
npm run tauri dev
```

## Configuration

The planned configuration source is a local `settings.json`; there are no keys at scaffold stage.

| Key | Type | Default | Required | Range | Purpose | Example |
| --- | --- | --- | --- | --- | --- | --- |
| `theme` | string | `system` | No | `system` / `light` / `dark` | UI theme | `dark` |
| `language` | string | `zh-CN` | No | `zh-CN` / `en-US` | UI language | `zh-CN` |
| `tools.external.ffmpeg` | string | auto-detect | No | executable path | ffmpeg path | `/usr/local/bin/ffmpeg` |

Precedence: `settings.json` over built-in defaults. Hot reload: some UI keys apply immediately; the rest need a restart.

## API and Modules

At scaffold stage only a placeholder Tauri command exists; there is no stable API.

```rust
#[tauri::command]
fn ping() -> String
```

Parameters: none. Returns: a fixed string. Error codes: none yet.

Example call:

```ts
import { invoke } from '@tauri-apps/api/core';
const pong = await invoke<string>('ping');
```

Rate limits: none.

## Project Layout

```text
NovaKit/
├── Cargo.toml           # Rust package definition
├── package.json         # Frontend scripts and dependencies
├── src/                 # Rust entry point
├── src-tauri/           # Tauri config, capabilities, icons
│   ├── tauri.conf.json  # Window, bundling, security config
│   ├── capabilities/    # Permission declarations
│   └── src/             # Desktop command layer
└── NovaKit-开发手册.md   # Product, architecture, acceptance manual
```

## Performance

No measurements at scaffold stage. Design goals (not measured):

| Metric | Goal | Note |
| --- | --- | --- |
| Cold start | Under 3 s | Tools load on demand, not all upfront |
| Installer size | Under 100 MB per platform | External components (7z/ffmpeg) shipped on demand |

## Security and Privacy

Data collection: none; config and local data stay on the machine.

Transport: no network requests by default; external component calls never build shell strings.

Permission model: plugin manifests declare capabilities, denied by default, granted explicitly at install.

Known risk: loading third-party plugins carries their own risk; install only from trusted sources.

Reporting: <ntars@qq.com>.

## Compatibility

| Dimension | Supported | Verified |
| --- | --- | --- |
| OS | macOS (first); Windows / Linux planned | macOS scaffold builds |
| Runtime | Rust edition 2024 | Yes |
| Desktop shell | Tauri 2 | Yes |
| Database | Local SQLite | Planned |

Not supported: mobile, web, 32-bit systems.

## Known Issues and Limitations

**Known issues**

| Issue | Scope | Workaround | Tracker |
| --- | --- | --- | --- |
| Scaffold only, no usable tools | All features | Wait for the roadmap | [NovaKit-开发手册.md](NovaKit-开发手册.md) |

**Current limitations**: business code not started; frontend dependencies not wired; no installer.

**Unimplemented**

| Feature | Reason | Status |
| --- | --- | --- |
| 36 tools | Scaffold stage | Scheduled for v0.1 |
| Process plugin protocol | Waiting for core stability | Defined only in 0.1, not open |
| Auto-update | Not in a release flow yet | Scheduled |

## Changelog

Versioning follows Semantic Versioning and Keep a Changelog. `0.1.0` is the scaffold version; there is no `CHANGELOG.md` yet.

## Roadmap

> The following are plans, not delivery commitments; the actual scope follows release records.

| Item | Status | Target |
| --- | --- | --- |
| Core tools (files, images, text) | Planned | 0.1.0 |
| System tools (clipboard, screenshot, processes) | Planned | 0.1.0 |
| External component detection and calls | Planned | 0.1.0 |
| Process plugin protocol | After core stabilizes | TBD |

## Contributing

- Setup: install the Rust 2024 toolchain, Node.js 18+, and Tauri 2 dependencies.
- Branching: feature branches off `main`.
- Commits: Conventional Commits with a descriptive subject.
- PRs: one change per PR, `cargo build` and `cargo test` green.
- Conduct: no GPL/AGPL/SSPL dependencies; new source files must carry an SPDX identifier.

## License

Copyright © 2026 Ntarstudio. Licensed under Apache-2.0 / MIT.

Licenses are layered by directory: code under Apache-2.0, docs, contracts, and the plugin protocol under MIT. See [LICENSE](./LICENSE) and [LICENSE-MIT](./LICENSE-MIT); the mapping is in [LICENSING.md](./LICENSING.md).
