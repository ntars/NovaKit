# NovaKit 开发手册

> 手册版本 **1.0**（Tauri 版） · 2026-11-25 · 覆盖 **M0→M7**（从空仓库到 1.0，再到发布后 90 天运营） · 面向「人 + 本地 qwen3-coder-next」
>
> 技术底座（全部实测，2026-11-25 23:00）：**Rust 1.100.0 / Cargo 1.100.0**（在 `~/.cargo/bin`，**登录 shell 里不在 PATH**）· **Tauri 2**（CLI 未安装）· **Node 22.22.2 / npm 10.9.7** · **macOS 27.2.1 arm64** · macOS SDK 27.2 由命令行工具提供（`/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk`）——**Tauri 不需要完整 Xcode** · `hdiutil`/`codesign`/`xcrun`/`clang` 齐备 · **`pkg-config` 未安装** · 无 Developer ID · 代理对 `crates.io` 外的部分源返回 502。
>
> 手册位置：本文件在 `~/Downloads/`。**C0.2 第一步把它复制成仓库根 `NovaKit-开发手册.md`，此后以仓库内那份为准**，只改一处。
>
> 一句话定位：**NovaKit 是一个 macOS 桌面工具箱——装完系统只装这一套，覆盖日常最基础的那批动作，离线可用、无广告、无遥测。**
>
> 交付边界：**只有 macOS 进「支持」列表**（Apple Silicon 优先）。Windows / Linux 保留可编译的形状，但没真机验证就不宣称支持。
>
> 核心信条：**工具要小、边界要硬、失败要说人话、装完就能用、坏了能退。**

---

## 0. 怎么用这本手册

### 0.1 生命周期地图（先看终点，再看这一步）

这本手册不是「功能清单 + 技术选型」，是一条**从空仓库到 1.0、再到发布后能自己活着**的路径。八个阶段，每段都有命令级退出标准；不过标准不进下一段。

| 阶段 | 目标 | 主要产出 | 退出标准（可验证） | 谁做 |
|---|---|---|---|---|
| **M0 定基** | 能编译、能测、门禁能跑、能开一个空窗 | cargo workspace、36 工具契约、25 错误码、11 条门禁、最小 Tauri 壳、`BASELINE.md` | `cargo xtask ci` 绿（未建成项打印 SKIP）；`tauri build --debug` 出 `.app` | 人 2 张 + 模型 9 张 |
| **M1 外壳** | 有壳可用，能开两个真工具 | 窗口、路由、命令面板、设置、IPC 层、启动自检 | 首页可达；命令面板能开工具；异常进错误页并留日志 | 模型 |
| **M2 文本** | 9 个文本类工具 | 模块 + 单测 + 空/忙/错三态 | 每工具 ≥3 例测试绿；10MB 文本 1.5s 内可读 | 模型 |
| **M3 文件** | 6 个文件类工具 | 模块 + 安全用例 | 路径穿越、符号链接、zip slip、炸弹用例全绿 | 模型 |
| **M4 图像/PDF/剪贴板** | 6 个工具 | 模块 + 权限降级 | 500MB 图不崩；拒绝屏幕录制后有正确提示 | 模型 + 1 人工 |
| **M5 媒体/系统/组件** | 6 个工具 + 组件桥 | 组件检测、播放、转码、7z、系统页 | 拔掉 ffmpeg/7z 后走通安装指引；结束进程可恢复 | 模型 + 1 人工 |
| **M6 插件与 0.1 发布** | 可交付的 0.1.0 | 插件宿主 + 示例插件 + README + dmg | 附录 A「v0.1 检查单」全勾 | 模型 + 3 人工 |
| **M7 1.0 与运营** | 可长期维护、可交接 | 分诊/热修/兼容矩阵/贡献者流程/1.0 产物 | 附录 A「v1.0 检查单」全勾；发布后 90 天无 S0 | 模型 + 3 人工 |

三条关于阶段的事实：

1. **M0 不完成，后面全是空转**。现在 `cargo` 不在 PATH、tauri CLI 没装、`pkg-config` 缺失，三件都不做，`tauri build` 一定失败。所以 C0.1 是硬阻断。
2. **M6 不是终点**。多数项目停在「0.1 能跑」，三个月后没人敢改——因为没分诊、没热修、没兼容矩阵。M7 才是「和大厂一样」的分界线。
3. **每个阶段结束必须有一次「全门禁 + 一次真人走查」**，不许连跳两个阶段才回顾。

### 0.2 现状盘点（先看清起点）

| 项 | 实测 | 对计划的影响 | 由谁产出 |
|---|---|---|---|
| 仓库 | 只有一份手册（在 Downloads），无 `.git`、无代码 | M0 第一张卡是 `git init` + 手册入库 + 首次提交 | C0.2 |
| Rust | 1.100.0（`~/.cargo/bin`，**不在 PATH**） | 不修 PATH，任何 `cargo` 命令都报 not found | C0.1（人工） |
| Node | 22.22.2 / npm 10.9.7 | 前端构建齐备，锁 `package-lock.json` | C0.2 |
| Tauri CLI | **未安装**（`cargo tauri` 报 no such command） | 用 `cargo install tauri-cli --version "^2"` 装一次，之后全走 `cargo xtask` | C0.1（人工） |
| macOS SDK | 27.2（命令行工具提供） | **不需要装完整 Xcode**，`clang`/`hdiutil`/`codesign` 已够打包 | — |
| `pkg-config` | **未安装** | 少数 `-sys` crate 会卡；先装，避免后面逐个踩 | C0.1（人工） |
| 系统版本 | macOS 27.2.1 | 兼容矩阵从 13+ 起算，本机只覆盖最新一档 | C7.2 |
| 签名 | 无 Developer ID | 0.1 出**未签名 dmg**，README 写清首次打开方式 | C6.4 |
| 网络 | 代理对部分源 502 | `cargo fetch` 可能要配镜像；配了什么写进 `BASELINE.md` | C0.1 |
| CI | 无 `.github/` | M0 不接 CI，本地 `xtask ci` 就是唯一真相；M7 才接 Actions | C7.4 |
| 可观测 | 无 | 零遥测，靠本地日志 + 诊断包 + 报障模板（15.6） | C1.7 / C6.8 |

> 这张表是「事实」，不是「计划」。任何一条变了，改这里 + `docs/BASELINE.md`，否则 `check-docs` 判红。

### 0.3 「完全落地」14 条判定

做到这 14 条才叫「像成熟产品」。少一条都只是「能跑」。

| # | 判定项 | 验收方式 |
|---|---|---|
| 1 | 能启动、能关、崩溃不静默 | 冷启动 < 2s；异常进错误页并给日志路径 |
| 2 | 36 个工具入口全部可达，无死链 | 命令面板逐个搜到并打开 |
| 3 | 每个工具有空态、忙碌态、错误态 | 逐个走查（附录 A） |
| 4 | 文件操作永不越界 | 构造 `../../etc/passwd`、符号链接、超长路径，全部拒绝并给人话 |
| 5 | 长任务可取消、可看进度 | 转码/压缩/哈希中途取消，进程真被杀掉 |
| 6 | 外部组件缺失时给可执行指引 | 卸载 7z 后打开解压，提示「去安装」而不是错误码 |
| 7 | 不联网也能用核心工具 | 断网启动，T0 工具全部可用 |
| 8 | 零遥测、零后台请求 | 抓包 10 分钟，除用户主动发起的 API 调试外无出站 |
| 9 | 设置与数据可迁移、可备份 | 导出/导入配置；DB 备份恢复一次成功 |
| 10 | 权限申请有解释、拒绝后能降级 | 拒绝屏幕录制，截图工具提示去哪开，不崩 |
| 11 | 安装包可交付 | dmg 可挂载、可拖入 Applications、卸载干净 |
| 12 | 干净机器按 README 从零可用 | 新用户目录 + 新 dmg，30 分钟内跑通 10 个工具 |
| 13 | 问题可定位 | 一条命令导出诊断包（版本/系统/最近日志/脱敏配置），报障不用来回问 |
| 14 | 可交接 | 新人按 README + 本手册，1 天内新增一个工具并全门禁绿 |

**第 4、5、6 条是工具箱类软件的生死线**：用户把整个磁盘交给它，一次越界就是事故，一次卡死就是卸载。第 13、14 条决定「三个月后这个项目还活着吗」。

### 0.4 三种读者

| 读者 | 读哪几章 | 用法 |
|---|---|---|
| 你（人） | 0、1、2、10.9、14、15、16、附录 A/F | 定范围、跑人工门、发版、决定「加不加这个工具」、决定「1.0 冻不冻」 |
| 本地模型 | 全文，但每次只喂「当次卡 + 模块契约 + 一个同类实现 + 一个测试」 | 一次一张卡，按 11.3 的 A/B/C/D 输出 |
| 门禁 `xtask` | 3、9、10、12 | 校验契约/文档/依赖/体积/许可证，判退出码 |

### 0.5 默认决策（改一条要写进 `BASELINE.md` 并全表复核）

| 项 | 决定 | 代价 |
|---|---|---|
| 外壳 | **Tauri 2**（系统 WebView，WKWebView） | macOS 版本决定 WebView 能力，老系统要降级 |
| 后端 | **Rust**，全部业务逻辑在 `crates/novakit-core` | 学习曲线；换来内存安全与可测 |
| 前端 | **React 19 + TypeScript + Vite**，只负责渲染 | 多一套构建链；换来 UI 迭代速度 |
| 通信 | 只有 `#[tauri::command]` 一条通道，契约在 `contracts/ipc.json` | 每加一个命令要改两处（由 `check-ipc` 强制） |
| 权限 | **不给 WebView 任何文件系统/shell 权限** | 前端不能直接读文件，全走 IPC |
| 支持平台 | **macOS 13+（arm64 优先）** | 跨平台缺陷本机复现不了 |
| 状态管理 | React `useSyncExternalStore` + 一个 store，不引 Redux/Zustand | 自己写 60 行；换来零魔法 |
| 本地库 | `rusqlite`（bundled）+ 手写 SQL + 显式迁移 | 没有 ORM，换来可审 |
| 配置 | 自研 `settings.json`（原子写 + `schemaVersion`），不用 store 插件 | 自己维护迁移 |
| 外部组件 | 不随包分发，检测系统已装或用户指定路径 | 首次使用多一步安装 |
| 插件 | 0.1 只定义**进程插件协议**，不开市场 | 生态慢，但不引任意代码执行 |
| 网络 | 除「API 调试」（用户主动发请求）外无出站 | 没有云同步、没有在线更新 |
| 语言 | 中文优先，代码与注释英文 | i18n 结构留好，1.0 前不做多语言 |
| 许可 | **Apache-2.0（代码）+ MIT（文档/协议/示例）分层** | 两套 SPDX 头与 NOTICE 要维护 |
| 发布 | 未签名 dmg + 手动下载；不做自动更新 | 用户要自己换版本 |

### 0.6 范围锁（三段，越界即停）

| 版本 | 必须包含 | 判定门槛 |
|---|---|---|
| **v0.1**（M6） | 36 个工具、主框架、设置、组件桥、进程插件协议 + 1 个示例插件 | 附录 A「v0.1 检查单」全勾 |
| **v0.5**（M7 前半） | 分诊流程、热修能力、兼容矩阵、诊断包、性能基线 | 模拟一次 S1，7 天内修完并发出补丁版 |
| **v1.0**（M7 后半） | 契约冻结（工具 ID / 配置字段 / DB schema / 插件协议）、贡献者流程、1.0 文档 | 附录 A「v1.0 检查单」全勾 + 冻结评审签字 |

**明确延后**：内嵌 Office 渲染、全文检索、SSH/Telnet、SQLite 查看器、屏幕录制、OCR/翻译、图标提取、音乐库管理、磁盘基准、键鼠测试、色盲模拟、插件市场、云同步、多语言、自动更新、移动端/Web 端。

**禁止（硬边界）**：联网账号体系、遥测/崩溃上报、自动更新器、捆绑安装器、系统级驱动、内核扩展、修改系统文件、以 root 运行、任何形式的「一键优化/清理注册表/加速开机」。

延后的理由只有一条：**每个功能都要有人长期维护它的失败路径。** 36 个已经够多了。

### 0.7 命名与术语

| 词 | 含义 | 不许混用 |
|---|---|---|
| 工具（tool） | 一个可独立打开的页面，唯一 ID 如 `json.format` | 不叫「模块」「功能」「插件」 |
| 模块（module） | `novakit-core` 里实现一个工具的一组文件 | 不叫「组件」 |
| 组件（component） | 外部可执行程序：ffmpeg / 7z | 不叫「依赖」「插件」 |
| 插件（plugin） | 第三方独立进程，走 JSON-RPC | 不叫「扩展」「组件」 |
| 命令（command） | `#[tauri::command]` 函数，IPC 的唯一入口 | 不叫「接口」「API」 |
| 卡（card） | 第 10 章的一条任务，ID 如 `C3.4` | 不叫「任务」「issue」 |
| 门禁（gate） | `cargo xtask` 的一条检查 | 不叫「测试」 |

---

## 1. 产品

### 1.1 名字与定位

- 名字：**NovaKit**（`nova` 新星 + `kit` 工具套件）；可执行文件名 `novakit`；bundle id `app.novakit.desktop`。
- 中文口号：**装完系统，只装这一套。** 英文：**One kit, day one.**
- 一句话：不是「全能软件」，是「一台新电脑上最先该装的那套小工具」。

> 命名风险：`Nova` 前缀在开发者工具里很拥挤。公开前做三件事：GitHub 组织名、crates.io / npm 包名、域名可用性。没做完不对外发版（附录 A 已列）。

### 1.2 用户与场景

目标用户不是极客，是**刚换电脑、不想装十个全家桶的人**。

| 场景 | 用户动作 | 涉及工具 |
|---|---|---|
| 新机第一天 | 装完系统，装 NovaKit，能解压、能看图、能截图 | T09/T11/T16 |
| 收到压缩包 | 先看目录再决定解到哪 | T10/T09 |
| 写配置/看日志 | 打开大文本、转编码、比两版差异 | T17/T22/T06 |
| 处理一堆照片 | 批量改名、压缩、转格式、拼长图 | T01/T12/T14 |
| 开发联调 | JSON 格式化、正则、时间戳、发个 GET | T21/T26/T27/T30 |
| 翻剪贴板历史 | `Cmd+Shift+V` 找刚复制的东西 | T33 |
| 装完软件清启动项 | 看开机自启、关掉不需要的 | T35 |

### 1.3 不做什么（写进 README 第一屏）

- **不是 Office 替代品**：不内嵌 Word/Excel/PPT 渲染，双击交给系统默认程序。
- **不是清理大师**：不删「垃圾」、不改系统设置、不写注册表。
- **不是下载器/浏览器/播放器全家桶**：播放器只放本地文件。
- **不联网**：没有账号、没有云同步、没有「猜你喜欢」。
- 不做移动端、不做 Web 版。

### 1.4 许可分层（Apache-2.0 + MIT）

这不是「二选一双许可」，而是**按目录分层**：需要专利与商标保护的部分用 Apache-2.0，需要零摩擦复用的部分用 MIT。

| 范围 | 许可 | 理由 |
|---|---|---|
| `crates/`、`src-tauri/`、`ui/src/`（除 `gen/`）、`xtask`、构建脚本 | **Apache-2.0** | 含专利授权（§3）与商标保留（§6）；贡献者条款清晰；公司采用时法务最容易过 |
| 第 4 章插件协议、`plugins/sample-*`、`docs/`、本手册、`README.md`、`contracts/*.json` | **MIT** | 第三方写插件、抄示例、改文档时不用管专利条款与 NOTICE 义务 |
| 第三方依赖 | 各自原许可 | 记录进 `THIRD-PARTY-LICENSES.md`，随包分发 |

四个必建文件（C0.2 产出，缺一 `check-licenses` 判红）：

| 文件 | 内容 | 谁读 |
|---|---|---|
| `LICENSE` | Apache-2.0 全文（不许改一字） | 法务、代码托管平台识别 |
| `LICENSE-MIT` | MIT 全文 + 首行注明「适用于 docs/、contracts/、plugins/、插件协议」 | 插件作者 |
| `NOTICE` | 项目名、版权年、第三方组件清单指引 | Apache-2.0 §4(d) 要求 |
| `LICENSING.md` | **分层映射表** + 一张「我要用你的代码，该看哪份」问答 | 所有人 |

四条硬规则：

1. **每个源文件头必须有 SPDX 标识**：`// SPDX-License-Identifier: Apache-2.0` 或 `// SPDX-License-Identifier: MIT`，且与所在目录映射一致。由 `check-licenses` 扫，不靠自觉。
2. **不引入 GPL / AGPL / SSPL 依赖**。它们会通过分发义务传染整个二进制。`check-licenses` 读 `Cargo.lock` + `package-lock.json` + `deny.toml` 三重拦截。
3. **商标不在许可范围内**：`NovaKit` 名称与图标不随代码授权（Apache-2.0 §6 已声明）。可以 fork 代码，不能叫 NovaKit。
4. **贡献用 DCO 不用 CLA**：提交带 `Signed-off-by:`。Apache-2.0 本身已含专利授权，再签一份 CLA 只会劝退贡献者。

发布包必须包含：`LICENSE`、`LICENSE-MIT`、`NOTICE`、`THIRD-PARTY-LICENSES.md`（放在 `.app/Contents/Resources/`），关于页显示「代码 Apache-2.0 · 文档与插件协议 MIT」。

> 为什么不一刀切 MIT：MIT 没有专利授权条款。工具箱会用到图像编解码、压缩、媒体解析，这些领域专利密集，Apache-2.0 §3 是给使用者的实质保护。
> 为什么不一刀切 Apache：让一个只想写 50 行插件的人去处理 NOTICE 与专利条款，等于把生态掐死在第一步。

### 1.5 版本路线（每个版本只承诺一件事）

| 版本 | 承诺 | 不承诺 |
|---|---|---|
| 0.1.0 | 36 个工具能用，安全边界立住 | 稳定 API、性能、Intel、旧系统 |
| 0.2–0.5 | 修真实报障；诊断包、兼容矩阵、性能基线补齐 | 新工具（除高频缺失） |
| 1.0.0 | 契约冻结：工具 ID、配置字段、DB schema、插件协议不再破坏性变更 | 跨平台支持、云功能、插件市场 |

> 0.x 期间允许破坏性变更，但**配置与 DB 必须迁移**，不许让用户数据失效。1.0 之后走 semver，破坏性变更进 2.0。

---

## 2. 工具矩阵

### 2.1 四层分级（决定实现方式与验收方式）

| 层 | 含义 | 实现 | 失败模式 |
|---|---|---|---|
| **T0 纯 Rust** | 无系统权限、无外部程序 | `novakit-core` 纯逻辑，单测可覆盖 | 输入非法、内存不足 |
| **T1 系统能力** | 需系统 API 或权限（截屏、剪贴板、进程、播放） | Rust crate（`arboard`/`sysinfo`）+ 系统命令（`screencapture`） | 授权被拒、API 不可用 |
| **T2 外部组件** | 依赖 ffmpeg / 7z | `Runner` 起子进程，解析输出 | 组件缺失、版本不符、进程被杀 |
| **T3 进程插件** | 第三方扩展 | JSON-RPC over stdio | 插件崩溃、协议不符、恶意行为 |

分层不是学术分类，是**验收方式不同**：T0 用 `cargo test` 就能证；T1 必须真机点授权；T2 必须拔掉组件试一次；T3 必须故意传坏消息。

### 2.2 v0.1 的 36 个工具

| ID | 工具 | 层 | 一句话验收 |
|---|---|---|---|
| T01 | 批量重命名 | T0 | 正则/序号/时间戳三种规则，先预览后执行，可撤销 |
| T02 | 重复文件查找 | T0 | 按大小预筛 + 哈希确认，列出可删清单不自动删 |
| T03 | 哈希校验 | T0 | 文本/文件算 MD5/SHA1/SHA256，可对照 `.sha256` 文件 |
| T04 | 文件分割/合并 | T0 | 分卷可合并还原，sha256 一致 |
| T05 | 磁盘空间分析 | T0 | 目录树 + 前 100 大文件，扫描可取消 |
| T06 | 文本比较 | T0 | 两文件行级 diff，忽略空白选项 |
| T07 | 文件名搜索 | T0 | 索引指定目录，200ms 内出结果，可停 |
| T08 | 文件名编码修复 | T0 | GBK 乱码名预览后转 UTF-8，冲突有策略 |
| T09 | 解压/压缩 | T2 | 7z 列目录、解压到指定目录、打包 zip/7z |
| T10 | 压缩包预览 | T2 | 不解压看目录，单个小文件可预览 |
| T11 | 图片查看 | T0 | 常见格式 + 缩放/旋转/上一张下一张 |
| T12 | 格式转换 | T0 | PNG/JPG/WebP/BMP/TIFF 互转，批量，保留目录结构 |
| T13 | 图片压缩 | T0 | 质量/尺寸可调，显示前后体积 |
| T14 | 图片批处理 | T0 | 缩放/旋转/裁剪/水印/长图拼接，预设可存 |
| T15 | 屏幕取色 | T1 | 截一帧取像素，显示 HEX/RGB/HSL，可复制 |
| T16 | 截图标注 | T1 | 区域/窗口/全屏，箭头/矩形/马赛克，复制或保存 |
| T17 | 纯文本编辑器 | T0 | 打开 >10MB 文件不卡死，编码/换行可转，保存前提示 |
| T18 | Markdown 编辑/预览 | T0 | 分栏预览，导出 HTML，图片相对路径可用 |
| T19 | 文本工具箱 | T0 | 去重/排序/大小写/统计/简繁/正则替换 |
| T20 | PDF 概览 | T1 | 文本提取 + 页数/尺寸/元信息 + 一键用系统预览打开 |
| T21 | JSON 格式化 | T0 | 格式化/压缩/校验/路径查询，大文件不冻界面 |
| T22 | 编码转换 | T0 | Base64/URL/Hex/HTML 实体/Unicode 互转 |
| T23 | 音视频播放 | T1 | 本地常见格式播放、倍速、字幕文件 |
| T24 | 音视频转码 | T2 | 预设（H.264/HEVC/GIF/抽音轨），进度可取消 |
| T25 | 媒体信息 | T2 | 编码/分辨率/码率/时长，ffprobe 输出结构化 |
| T26 | 正则测试 | T0 | 实时高亮、分组、替换预览 |
| T27 | 时间戳/时区 | T0 | Unix ↔ 本地/UTC，多时区对照 |
| T28 | 二维码 | T0 | 生成 PNG/SVG；从图片识别 |
| T29 | 颜色转换 | T0 | HEX/RGB/HSL/HSV 互转 + 调色板 |
| T30 | 简易 API 调试 | T0 | GET/POST、Header、JSON 响应美化，历史本地存 |
| T31 | 计算器/单位换算 | T0 | 科学计算 + 程序员模式 + 常用单位 |
| T32 | 密码生成 | T0 | 长度/字符集/排除易混字符，强度提示 |
| T33 | 剪贴板历史 | T1 | 文本历史 200 条，搜索、置顶、一键复制 |
| T34 | 便签/待办 | T0 | 本地存储，标签过滤，导出 Markdown |
| T35 | 系统信息 | T1 | 硬件/系统/磁盘概览 + 开机启动项查看与关闭，可复制诊断信息 |
| T36 | 进程管理 | T1 | 列表、排序、结束进程（二次确认） |

> 每个工具都必须有：空态文案、失败文案（人话）、`Esc` 返回、`Cmd+W` 关闭当前工具页、且能在命令面板被搜到。

### 2.3 从「装完系统第一周」反推，补齐 12 项

按「真实第一周会用到什么」筛，补齐了 12 个基础动作。它们已并入 2.2（标 ★；「开机启动」并入 T35）：

| # | 补充 | 为什么是基础 | 落在 |
|---|---|---|---|
| 1 | ★ 纯文本编辑器 | 没有它，「看日志/改配置」只能开终端；这是工具箱的地基 | T17 |
| 2 | ★ 文本工具箱 | 去重、排序、统计是每天都会用的动作，比正则测试高频 | T19 |
| 3 | ★ 文件名搜索 | 「全文检索」太重，先解决 90% 的「我记得文件名」 | T07 |
| 4 | ★ 文件名编码修复 | 老硬盘/压缩包里的 GBK 乱码是真实痛点，别处没得修 | T08 |
| 5 | ★ 颜色转换 | 取色器只解决「取」，不解决「换」；设计/前端都要 | T29 |
| 6 | ★ 图片批处理 | 只有转换和压缩不够，缩放/水印/长图才是批量刚需 | T14 |
| 7 | ★ 压缩包预览 | 「先看再解」能避免解出一地垃圾，也是安全边界 | T10 |
| 8 | ★ 大文件/目录占用 | 磁盘满了第一反应是「谁占的」，必须能定位到文件 | T05 |
| 9 | ★ 音视频转码 | 只有「转 GIF」太窄；压视频、抽音轨才是常用 | T24 |
| 10 | ★ 开机启动管理 | 新机装完软件必做的一件事，系统设置藏得深 | T35 |
| 11 | ★ 条码/二维码识别 | 只有生成不够，收到图要能读出来 | T28 |
| 12 | ★ 系统信息与诊断 | 报障时「你的环境是什么」必须有现成答案 | T35 |

未补但考虑过的：十六进制查看、空格预览（Quick Look 式）、SSH、SQLite 查看器、屏幕录制、OCR、图标提取——都属「有用但不基础」，留在延后清单，避免 0.1 变成半成品博览会。

### 2.4 为什么砍掉「内嵌 Office 查看」

三个事实让它不适合 0.1：

1. 一个能渲染 docx/xlsx/pptx 的引擎，体积与首启时间都远超一个工具箱的预算。
2. 这类引擎的许可证（MPL 系）与「按需下载」的分发方式要额外合规工作。
3. 系统默认程序已经能开，**工具箱的价值是补充，不是替代**。

0.1 的做法：文件页对 Office 文件提供「用系统默认程序打开」，README 说明。真要内嵌，1.0 之后单开评估。

### 2.5 八条取舍，逐条有理由

| 项 | 处理 | 理由 |
|---|---|---|
| 插件市场 | 只定协议，不开市场 | 市场要签名、审核、撤回机制，是另一个产品 |
| 内嵌 Office 查看 | 交系统默认程序 | 体积与许可证双重成本 |
| 全文检索 | 先做文件名索引 | 内容索引占空间、涉隐私，收益远低于成本 |
| SSH/Telnet | 延后 | 需要完整终端模拟器（xterm 兼容），是另一个产品 |
| 屏幕录制 / GIF | 延后 | 权限、编码、性能三座山，0.1 只做截图 |
| 音乐库管理 | 只做文件播放 | 曲库刮削/歌词/均衡器是长期维护负担 |
| 云同步 / 账号 | 禁止 | 与「离线、无账号」定位冲突 |
| 多语言 | 结构预留，中文优先 | i18n 不是翻译文件，是文案与布局的返工 |

### 2.6 新增一个工具的四道闸（决定「加不加」，不是「怎么加」）

加工具的流程在附录 C；**能不能加**先过这四道闸，任何一道不过就写进延后清单：

| 闸 | 问题 | 不过的例子 |
|---|---|---|
| 1 频次 | 目标用户一个月会用一次以上吗 | 色盲模拟、磁盘基准 |
| 2 唯一性 | 系统自带或已有工具能替代吗 | 计算器（Spotlight 能做，但高频且离线，保留） |
| 3 失败路径 | 它的失败路径有人长期维护吗（超时/坏文件/权限） | 屏幕录制（编码 + 权限 + 性能三份长期债） |
| 4 安全面 | 它是否要求新权限、新出站、新原生库 | 在线翻译（出站）、系统驱动 |

> 加一个工具的真实成本不是 300 行代码，是**它未来三年的失败路径**。四道闸就是把这笔账提前算。

---

## 3. 架构

### 3.1 总览

```
┌───────────────────────────────────────────────────────────────┐
│ WebView（React 19 + TS + Vite）——只有渲染                         │
│  ui/  页面与组件   tools/  36 个工具页   gen/  IPC 类型（生成物）  │
│   · 无 fs / 无 shell / 无 fetch / 无 localStorage                │
└──────────────────────────┬────────────────────────────────────┘
                           │ 唯一通道：invoke('cmd_*')  ← contracts/ipc.json
┌──────────────────────────▼────────────────────────────────────┐
│ src-tauri（薄壳）                                                │
│  commands/  每个命令 ≤ 10 行：取 state → 调 core → 映射错误        │
│  state.rs   AppState（DB、设置、JobQueue、Runner 单例）           │
│  events.rs  job:progress / job:done / toast 事件发射              │
│  capabilities/default.json  最小权限集（无 fs、无 shell）         │
└──────────────────────────┬────────────────────────────────────┘
                           │ 普通 Rust 调用（core 不依赖 tauri）
┌──────────────────────────▼────────────────────────────────────┐
│ crates/novakit-core（全部业务）                                   │
│  fs/ 路径闸门·原子写·临时目录·废纸篓   proc/ Runner（超时/取消/上限）│
│  db/ rusqlite + 迁移     settings/ 原子写 + 迁移                  │
│  jobs/ 队列·进度·取消     diag/ 诊断包     text/ 编码·diff·模糊     │
│  tools/<id>/            36 个工具的逻辑（纯函数优先）              │
│  plugins/               进程插件宿主（JSON-RPC over stdio）        │
└───────────────────────────────────────────────────────────────┘
                           │
        ~/Library/Application Support/NovaKit/
          settings.json  novakit.db  logs/  cache/  trash/  diag/  plugins/
```

**这张图最重要的不是分层，是那条 `core` 不依赖 `tauri` 的线**：业务逻辑放不进 UI、UI 也碰不到系统，不是靠约定，是靠**编译期事实**——`novakit-core` 的 `Cargo.toml` 里没有 `tauri`，想写也写不进去。

### 3.2 三条硬边界

| # | 边界 | 规则 | 违反后果 |
|---|---|---|---|
| 1 | 前端不碰系统 | WebView 无 `fs`/`shell` 权限；前端只 `invoke`；禁 `fetch`/`localStorage`/`indexedDB` | 权限与错误处理散落，无法测 |
| 2 | 所有路径过闸 | 用户输入的路径先过 `SafePath::resolve()`：绝对化、`..` 归一、拒符号链接越界、拒 `/System` | 一个 `../` 就能删错文件 |
| 3 | 外部程序只经 `Runner` | 禁裸 `std::process::Command`；统一超时、输出上限、取消、日志 | 卡死、内存爆、参数注入 |

三条都由门禁扫（9.3）+ 运行期封死：第 1 条由 `capabilities` 与 CSP 保证，第 2、3 条由 CI 的 grep 保证。

### 3.3 模块契约（36 个工具统一形状）

Rust 侧：一个工具 = 一个目录 + 一个 trait 实现 + 纯函数逻辑。

```rust
// crates/novakit-core/src/tools/mod.rs
pub trait Tool: Send + Sync {
    fn id(&self) -> &'static str;          // "json.format"，点分，全局唯一
    fn title(&self) -> &'static str;       // "JSON 格式化"
    fn keywords(&self) -> &'static [&'static str];
    fn tier(&self) -> Tier;                // T0/T1/T2/T3
    fn run(&self, ctx: &ToolCtx, input: Value) -> Result<Value, NkError>;
}
```

`ToolCtx` 是工具唯一能碰到外部世界的入口（等价于依赖注入，便于测试）：

```rust
pub struct ToolCtx<'a> {
    pub fs: &'a dyn SafeFs,
    pub runner: &'a dyn Runner,
    pub jobs: &'a JobQueue,
    pub settings: &'a SettingsStore,
    pub diag: &'a DiagSink,
}
```

规则：工具之间**不许互相调用**；共享逻辑上移到 `core/{fs,proc,text}`。这条由 `check-imports`（crate 内模块图）扫。

前端侧：一个工具 = 一个路由 + 一个页面组件。

```ts
// ui/src/tools/types.ts
export interface ToolPage {
  id: string;                 // 与 tools.json 一致，由 check-ipc 校验
  title: string;
  keywords: string[];
  component: React.ComponentType<{ host: ToolHost }>;
}

export interface ToolHost {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  toast(msg: string): void;
  openLogFolder(): void;
}
```

> 前后端各有一个「工具形状」，中间靠 `contracts/tools.json` 对齐：ID 只能来自那一份文件，`check-ipc` 双向比对。

### 3.4 依赖锁

版本在 `Cargo.toml` / `package.json` 写死到 minor；`Cargo.lock` 与 `package-lock.json` **入库**；升级只允许 C7.5 专卡。

**Rust 侧（`crates/novakit-core`）**

| 用途 | crate | 说明 |
|---|---|---|
| 序列化 | `serde` + `serde_json` | 契约、IPC、错误都用它 |
| 错误 | `thiserror` | `NkError` 的来源；不用 `anyhow`（会丢错误码） |
| 异步 | `tokio`（feature: `process`, `fs`, `time`, `sync`） | 只给 Runner/JobQueue 用 |
| 取消 | `tokio-util`（`CancellationToken`） | 长任务取消的唯一手段 |
| DB | `rusqlite`（feature `bundled`） | 自带 SQLite，免系统依赖 |
| 日志 | `tracing` + `tracing-appender` | 滚动 5MB×3 |
| 哈希 | `sha2` / `sha1` / `md-5` / `hex` | — |
| 编码 | `encoding_rs` + `chardetng` | GBK/UTF-16 与检测 |
| 图片 | `image` | 纯 Rust 编解码，转换/压缩/批处理都用它 |
| 二维码 | `qrcode`（生成）+ `rqrr`（识别） | 纯 Rust |
| diff | `similar` | 行级比较 |
| Markdown | `pulldown-cmark` + `ammonia` | 解析 + **必须脱敏**后再交给前端 |
| YAML/XML | `serde_yaml` / `quick-xml` | 解析与格式化 |
| 时间 | `chrono` + `chrono-tz` | 时区对照 |
| 正则 | `regex` | — |
| 表达式 | `meval` | 计算器；不自己写解析器 |
| 系统 | `sysinfo` | 进程、内存、磁盘 |
| 剪贴板 | `arboard` | 轮询与写入 |
| 目录遍历 | `walkdir` + `ignore` | 跳过 `.git`、`node_modules` |
| 网络（仅 API 调试） | `reqwest`（`rustls-tls`，禁默认 openssl） | 只允许 `tools/http.rs` 使用 |
| 单实例/托盘等 | 官方插件 | 见下 |

**Tauri 官方插件（只装用得上的）**

| 插件 | 用途 | 权限（capabilities） |
|---|---|---|
| `tauri-plugin-dialog` | 打开/保存对话框 | `dialog:allow-open`、`dialog:allow-save` |
| `tauri-plugin-clipboard-manager` | 前端「复制」按钮 | `clipboard-manager:allow-write-text` |
| `tauri-plugin-opener` | 用系统程序打开文件/文件夹 | `opener:allow-open-path` |
| `tauri-plugin-window-state` | 记住窗口大小位置 | `window-state:default` |
| `tauri-plugin-global-shortcut` | `Cmd+Shift+V` 唤起 | `global-shortcut:allow-register` |
| `tauri-plugin-single-instance` | 二次启动聚焦已有窗口 | `single-instance:default` |

**不装**：`tauri-plugin-fs`（前端不许直接读写）、`tauri-plugin-shell`（外部程序只走 Runner）、`tauri-plugin-http`（网络只在 Rust 侧）、`tauri-plugin-updater`（不做自动更新）、任何统计/崩溃 SDK。

**前端侧（`ui/package.json`）**

| 用途 | 包 | 说明 |
|---|---|---|
| 框架 | `react` + `react-dom` | 只渲染 |
| 构建 | `vite` + `@vitejs/plugin-react` + `typescript` | — |
| IPC | `@tauri-apps/api` | 只用 `core.invoke` 与 `event.listen` |
| 编辑器 | `codemirror` + `@codemirror/lang-json` + `@codemirror/lang-markdown` | 只给 T17/T18/T21 |
| 测试 | `vitest` + `@testing-library/react` | 组件与纯函数 |

**不引**：`axios`/`swr`/`react-query`（网络在 Rust）、`redux`/`zustand`（60 行 store 够）、`lodash`（按需写 3 行）、`moment`（`Intl` 够）、任何 UI 组件库（观感要自己控）。

> 依赖锁的真正作用：**堵住模型「顺手加个包」的手**。它不知道某个包已经两年没更新、或带了一个 30MB 的原生库。新包必须走人工卡。

### 3.5 目录结构（M0 建好，之后不许散建）

```
/Users/ntars/Documents/Github/NovaKit/
├─ NovaKit-开发手册.md          # 本文档，唯一开发文档
├─ AGENTS.md                    # 给模型的硬规矩（附录 E）
├─ README.md                    # 唯一用户文档（C6.3 写）
├─ CHANGELOG.md                 # Keep a Changelog
├─ LICENSE  LICENSE-MIT  NOTICE  LICENSING.md
├─ Cargo.toml                   # workspace + [workspace.package] version
├─ Cargo.lock                   # 入库
├─ rust-toolchain.toml          # 1.100.0，固定
├─ deny.toml                    # cargo-deny：许可证与安全公告
├─ contracts/
│  ├─ tools.json                # 36 个工具（id/title/keywords/tier）
│  ├─ errors.json               # 25 个错误码
│  ├─ exit-codes.json           # 4 个退出码
│  └─ ipc.json                  # 命令名 + 请求/响应类型
├─ crates/
│  ├─ novakit-core/             # 业务逻辑，不依赖 tauri
│  └─ novakit-xtask/            # 门禁（bin: xtask）
├─ src-tauri/                   # package: novakit-app
│  ├─ Cargo.toml  build.rs
│  ├─ tauri.conf.json
│  ├─ capabilities/default.json
│  ├─ icons/
│  └─ src/{main.rs, lib.rs, state.rs, events.rs, commands/*.rs}
├─ ui/
│  ├─ package.json  package-lock.json
│  ├─ vite.config.ts  tsconfig.json  index.html
│  └─ src/{main.tsx, app/, ui/, tools/, gen/, styles/}
├─ tests/                       # 跨 crate 集成测试
├─ plugins/sample-hash/         # 示例插件（Rust 或任意语言的可执行文件）
├─ docs/
│  ├─ BASELINE.md  EVIDENCE.md  TRIAGE.md  COMPAT.md  PERF.md
│  ├─ UPGRADE.md   FREEZE.md    HANDOFF.md RUNLOG.md
├─ assets/                      # 图标、示例文件（≤2MB）
└─ dist/                        # 构建产物，gitignore
```

### 3.6 四条数据流

**A. 普通工具（同步、快）**

```
用户操作 → invoke('cmd_text_json_format', {input})
  → commands/text.rs（≤10 行）→ core::tools::json::run()
  → 返回 serde_json::Value → 前端渲染
失败 → NkError { code, name, message, action } → invoke 的 reject → 统一错误条
```

**B. 长任务（可取消）**

```
前端 invoke('cmd_job_start', {tool, spec})
  → JobQueue 入队（并发上限 2）→ 立刻返回 jobId
  → 后台跑：每个进度点 emit('job:progress', {jobId, percent, msg})
  → 前端 listen 更新进度条；点取消 → invoke('cmd_job_cancel', {jobId})
  → CancellationToken.cancel() → Runner kill(SIGTERM)→2s→SIGKILL
  → emit('job:done', {jobId, status}) + 写 jobs 表（最近 100 条）
```

**C. 外部组件**

```
检测（settings 指定路径 → PATH → 常见安装位置）→ 有则直接用
  → 无 → 组件页显示版本/许可证/体积 → 用户点「选择文件」或「看安装说明」
  → 记录路径与版本 → 调起前跑 `-version` 校验，不符给降级提示
```

**D. 诊断与崩溃**

```
panic / 未捕获错误 → 写 logs/crash-<ts>.log（backtrace + 版本 + 系统）
  → 前端显示错误页：人话 + 「复制诊断包」+「打开日志目录」
  → 复制诊断包 → core::diag 生成 diag-<ts>.zip（白名单，见 8.4）
  → 全程不出网
```

### 3.7 版本与兼容

| 版本 | 载体 | 规则 |
|---|---|---|
| 产品版本 | `Cargo.toml` 的 `[workspace.package] version` | **唯一源**；`tauri.conf.json` 不写 `version`（继承 crate）；`ui/package.json` 由 `check-manifest` 校验同值 |
| 配置 schema | `settings.json.schemaVersion` | 只增；读旧版必须迁移 |
| 数据库 schema | `PRAGMA user_version` | 只增；迁移按序执行 |
| 插件协议 | `manifest.protocol` | `1`；不兼容变更升主版本并拒绝加载 |
| IPC 契约 | `contracts/ipc.json` | 命令只增不删；删命令要跨大版本并保留一个版本的弃用告警 |
| 工具 ID | `contracts/tools.json` | 发布后不改；换实现不换 ID |

### 3.8 长任务模型（`JobQueue` 细则）

| 项 | 规则 | 为什么 |
|---|---|---|
| 并发 | 最多 2 个；第 3 个排队，不拒绝 | 拒绝会让用户以为坏了；无上限会让磁盘抖到卡死 |
| 进度节流 | 每 100ms 最多 emit 一次 | 逐字节 emit 会把 IPC 打满、UI 卡住 |
| 取消语义 | `cancel()` 后 2s 内必须停；子进程 `SIGTERM` → 2s → `SIGKILL` | 「取消了还在跑」比不取消更糟 |
| 重试 | **不自动重试**。失败给「重试」按钮，用户点才重跑 | 自动重试会把磁盘写满 |
| 历史 | 写 `jobs` 表，保留 100 条 | 报障时能看「他到底点了什么」 |
| 结果 | 取消/失败都不留半成品：临时目录整体删 | 半截文件是最难解释的 bug |
| 线程 | 阻塞 IO 一律 `spawn_blocking`，禁止在主线程做文件操作 | 主线程一卡，整个窗口假死 |

> 这一节是给模型的「模板级约束」：新增任何长任务都复用 `JobQueue`，**不许自己开线程或裸 `tokio::spawn` 乱跑**。

---

## 4. 进程插件协议（0.1 只定义，不开放市场）

### 4.1 为什么是「进程插件」

WebView 里跑的是 JS，宿主是 Rust，两者都不适合加载第三方原生代码：动态库一崩就是整个 App 崩，而进程插件可以限时、可以杀、可以限输出、可以不给网络。代价是每个插件带一个可执行文件（首次启动慢 50–200ms），可接受。

### 4.2 manifest.json（唯一入口，缺字段拒绝加载）

```json
{
  "id": "dev.novakit.sample.hashplus",
  "name": "Hash Plus",
  "version": "0.1.0",
  "protocol": 1,
  "description": "批量哈希与校验",
  "entry": { "macos-arm64": "bin/hashplus" },
  "permissions": ["fs.read.selected", "fs.temp"],
  "limits": { "timeoutMs": 30000, "maxOutputBytes": 1048576 }
}
```

规则：

- `id` 必须反域名，全局唯一，加载后不可改。
- `entry` 按「平台-架构」给相对路径；没有当前平台条目 → 显示「不支持此平台」，不报错。
- `protocol` 不等于宿主支持范围 → 拒绝加载并提示升级。
- 多余字段允许忽略；缺字段一律拒绝，不许「猜默认值」。

### 4.3 通信：JSON-RPC 2.0 over stdio（一行一条）

| 方向 | 方法 | 说明 |
|---|---|---|
| 宿主 → 插件 | `initialize` | 传 `protocol`、`locale`、`tmpDir`、已授权 permissions；插件返回 `{name, version, tools[]}` |
| 宿主 → 插件 | `run` | `{toolId, input, requestId}`；长任务立刻返回，进度走通知 |
| 宿主 → 插件 | `cancel` | `{requestId}`；插件须在 2s 内退出当前任务 |
| 宿主 → 插件 | `shutdown` | 退出；宿主 3s 后强杀 |
| 插件 → 宿主 | `progress` | 通知：`{requestId, percent, message}` |
| 插件 → 宿主 | `log` | 通知：仅进日志，不弹窗 |
| 插件 → 宿主 | `result` / `error` | 与请求 `id` 对应；错误码用宿主定义的字符串 |

硬约束：

1. 单条消息 ≤ 1 MiB，超限断开并记 `5004`。
2. 插件**不得**写 stdout 打日志（会破坏协议）；日志走 stderr 或 `log` 通知。
3. 宿主只传白名单环境变量（`PATH`、`HOME`、`TMPDIR`、`LANG`），其余清空。
4. 超时（默认 30s，长任务按 `limits.timeoutMs`）到点 `SIGTERM`，2s 后 `SIGKILL`。
5. 插件进程的 `cwd` 固定为插件目录，禁止 `..` 逃逸。

### 4.4 权限模型（默认全拒）

| 权限 | 允许做什么 | 0.1 是否可授 |
|---|---|---|
| `fs.read.selected` | 只读用户在本工具里选中的路径 | 是 |
| `fs.write.selected` | 只写用户选中的输出路径 | 是 |
| `fs.temp` | 读写自己的临时目录 | 是 |
| `clipboard.read` | 读剪贴板 | 否（协议保留） |
| `net` | 出站网络 | **否，协议保留** |
| `proc.spawn` | 再起子进程 | **否** |

任何插件请求未授权能力 → `5002`，并把该插件标为「受限」。安装界面必须逐条显示权限，用户点「同意」才算装。

### 4.5 安装与信任

- 只支持「从文件夹安装」：用户选目录 → 校验 manifest → 计算 `entry` 的 sha256 → 记录到 DB → 复制到 `plugins/<id>/`。
- 不做自动下载、不做静默更新。更新 = 重新安装 = 用户再点一次。
- 首次运行弹一次「这是第三方程序，拥有你授予的文件权限」确认。
- 卸载 = 删目录 + 删记录；插件数据在 `plugins/<id>/data`，一并删需二次确认。

### 4.6 示例插件（C6.2 产出，必须能跑）

`plugins/sample-hash/`：一个约 60 行的 Rust bin（`cargo build --release`），实现 `run` 返回一个文件的 sha256。用途不是功能，是**证明协议可用**：C6.2 验收就是主程序能装上它、调通、超时能杀、坏消息能拒。

### 4.7 插件复审清单（每次改协议都过一遍）

| # | 检查 | 不做的后果 |
|---|---|---|
| 1 | 新增字段可缺省吗？缺省行为安全吗？ | 老插件全部加载失败 |
| 2 | 新权限默认拒绝吗？ | 静默提权 |
| 3 | 超时与输出上限仍生效吗？ | 一个插件拖垮整个 App |
| 4 | 崩溃只影响该插件吗？ | 主进程陪葬 |
| 5 | 协议版本不匹配时「拒绝 + 说明」吗？ | 用户看到「闪退」 |
| 6 | 日志里可能写入用户文件内容吗？ | 隐私事故 |

## 5. 数据与配置

### 5.1 settings.json（唯一配置源）

位置：`~/Library/Application Support/NovaKit/settings.json`。写入必须原子（写 `.tmp` → `rename`），权限 0600。

```json
{
  "schemaVersion": 1,
  "theme": "system",
  "accent": "#4C8DFF",
  "locale": "zh-CN",
  "lastTool": "json.format",
  "favorites": ["json.format", "text.editor"],
  "recentFiles": [{ "path": "/Users/x/a.json", "tool": "json.format", "at": "2026-11-25T14:00:00Z" }],
  "components": { "ffmpeg": { "path": "/opt/homebrew/bin/ffmpeg", "version": "7.1" } },
  "limits": { "maxOpenFileMB": 200, "clipboardItems": 200, "jobHistory": 100 },
  "privacy": { "clipboardHistory": false, "fileIndexRoots": [] },
  "localStats": { "launches": 0, "toolOpens": {} }
}
```

规则：

- 未知字段保留（`#[serde(flatten)] extra`），读取时忽略，写回时原样带出。
- 损坏 → 备份为 `settings.json.bad-<ts>`，用默认值启动，提示一次，不静默清空。
- 迁移函数 `migrate(from: u32, value: &mut Value) -> Result<(), NkError>`，每跨一版一个函数，必须有过测试的固定输入样例。
- 任何字段变更必须同步 `contracts/settings.schema.json`（9.5 的 `check-docs` 会比对键名）。

### 5.2 数据库（`novakit.db`，4 张表）

| 表 | 用途 | 关键字段 | 清理策略 |
|---|---|---|---|
| `clipboard` | 剪贴板历史 | `id, kind, text, hash, pinned, created_at` | 最多 200 条，置顶不淘汰 |
| `file_index` | 文件名索引 | `path, name, dir, size, mtime, root` | 按根目录重建；上限 20 万条 |
| `jobs` | 长任务历史 | `id, tool, input, status, started_at, ended_at, error` | 最近 100 条 |
| `notes` | 便签/待办 | `id, title, body, tags, done, updated_at` | 用户自管 |

迁移：`PRAGMA user_version` 从 0 起，`0001_init.sql`… 按序执行，每个文件包在一个事务里；执行完把文件名的 sha256 写进 `migrations` 表，**改已提交的 SQL 一律红**。

### 5.3 数据边界

- 不写系统目录、不写其他 App 的容器目录。
- 缓存与日志可一键清空；清空不动 `settings.json` 与 `novakit.db`。
- 导出配置 = 只导 `settings.json`（不含历史与索引），导入时校验 `schemaVersion`。
- 卸载说明必须列出残留路径（`~/Library/Application Support/NovaKit/`）。

### 5.4 备份、恢复与迁移演练

「可恢复」不是文档承诺，是**练过的动作**。每个版本发布前跑一次：

| 动作 | 步骤 | 验收 |
|---|---|---|
| 备份 | 退出 App → 复制 `settings.json` + `novakit.db` 到 `~/Desktop/novakit-backup-<date>/` | 两文件都在，且 `sqlite3 novakit.db .tables` 能列出 4 张表 |
| 恢复 | 删除 App Support 目录 → 放回两文件 → 启动 | 设置、便签、剪贴板历史都在 |
| 迁移 | 用上一版启动 → 升级到新版 → 查 `user_version` 与 `schemaVersion` 自增 | 无 `4002`，旧数据可读 |
| 回滚 | 用新版写数据 → 换回旧版启动 | **预期失败**，README 必须写「回滚前先恢复备份」 |

> 最后一行是重点：迁移只前不后。假装能回滚，比明确说不能回滚更危险。

### 5.5 本地使用计数（零遥测下怎么知道谁在用）

只写 `settings.json.localStats`，不联网、不聚合、不上报：

- `launches`：启动次数。
- `toolOpens`：每个工具被打开的次数（判断「哪个工具三个月没人开」）。
- 「关于」页展示一份「我的使用概览」，并给「清除统计」按钮。

用途只有一个：**你自己决定砍哪个工具、优先修哪个**。它不是遥测；哪天想上传，就越界了（12 章禁区第 1 条）。

---

## 6. 外部组件（T2）

### 6.1 组件清单与许可证

| 组件 | 用途 | 许可证 | 能否随包 | 0.1 策略 |
|---|---|---|---|---|
| `ffmpeg` / `ffprobe` | 转码、媒体信息 | LGPL-2.1+；Homebrew 构建启用 `--enable-gpl`，实际 **GPL-2.0+** | **不随包** | 检测系统已装；否则给安装指引 |
| `7z` | 压缩/解压/预览 | LGPL + **unRAR 限制条款** | **不随包** | 同上；不支持创建 RAR |

三条硬规矩：

1. **不打包、不静默下载**。不分发二进制 = 不承担 GPL 的分发义务；一旦随包，整个 App 的许可策略都要重做（见 1.4）。这不是洁癖，是许可边界。
2. **不碰 RAR 创建**；解 RAR 依赖系统 7z 的能力，失败就说「请用官方工具」。
3. 关于页必须列「本软件使用了哪些外部程序、各自许可证、如何卸载」，并链接 `THIRD-PARTY-LICENSES.md`。

### 6.2 检测顺序（结果写进设置）

```
1. settings.components[name].path（用户手动指定）
2. 按 PATH 逐目录查找可执行文件（不调 shell，不走 `which`）
3. 常见位置：/opt/homebrew/bin、/usr/local/bin
4. 都没有 → 组件页显示「未安装」+ 一条可复制的安装命令
```

每次调用前跑 `<bin> -version`（7z 用 `i`）校验；版本低于下限 → 提示升级，但**允许用户强制使用**（记 `degraded`）。

### 6.3 参数构造（不许拼 shell 字符串）

- `Runner` 内部一律 `Command::new(bin).args([...])`，**不经 shell**。
- 文件名作为独立参数传入，永远不拼进一个字符串。
- 输出目录必须是用户选的目录 + 程序生成的子目录，不接受用户输入的相对路径直接落盘。
- 进度解析：ffmpeg 用 `-progress pipe:1 -nostats`，只读 `out_time_ms` / `progress`；7z 用 `-bsp1` 读百分比；解析失败不算失败，退化为不确定进度。

### 6.4 组件缺失时的用户路径（必须走一遍）

打开「视频转码」→ 检测不到 ffmpeg → 页面显示：

> 需要 FFmpeg 才能转码。
> 1. 用 Homebrew 安装：`brew install ffmpeg`（可复制）
> 2. 或点「选择已安装的 ffmpeg」手动指定
> 不装也能用其他 32 个工具。

**不许**弹一个 `3001` 就完事。错误码进日志，人话进界面。

### 6.5 组件版本矩阵与升级

| 组件 | 下限 | 实测推荐 | 低于下限 | 更高版本 |
|---|---|---|---|---|
| ffmpeg | 5.0 | 7.x | 提示升级，允许强制（记 degraded） | 直接用；转码失败时提示「可能是新版本参数变化」 |
| 7z | 21.0 | 24.x | 拒绝（老版本不支持 `-bsp1` 进度） | 直接用 |

每次组件升级后，**对应工具必须重跑一遍集成测试**（C7.5 有清单）。版本兼容不是「应该没事」，是「跑过一次」。

---

## 7. 界面规范

### 7.1 布局

```
┌──────────────────────────────────────────────────────┐
│  🔍 搜索工具、文件、动作…            ⌘K      ⚙  ◐    │  ← 顶栏 48px
├──────────┬───────────────────────────────────────────┤
│ 文件      │                                           │
│ 图像      │        当前工具内容区                      │
│ 文本      │        （每工具独立滚动，不整页滚）         │
│ 媒体      │                                           │
│ 开发      │                                           │
│ 系统      │                                           │
│ ★ 收藏    │                                           │
├──────────┴───────────────────────────────────────────┤
│ 状态：就绪 · 任务 1 进行中 32%      v0.1.0 · 日志      │  ← 状态栏 28px
└──────────────────────────────────────────────────────┘
```

- 左栏 200–240px，可折叠；分组标题不可点。
- 首页是卡片网格（收藏 + 最近使用 + 全部），不是空白页。
- 单窗口多标签不做；一个工具一页，`Cmd+W` 回首页。
- 窗口最小 900×600；用 `window-state` 插件记住上次尺寸与位置。

### 7.2 命令面板（唯一必须做好的交互）

- `Cmd+K` 打开，输入即搜：工具名（中/英）、关键词、最近文件、动作（「打开日志目录」「清空剪贴板」）。
- 排序：精确前缀 > 关键词命中 > 模糊；每组最多 5 条，回车执行，`Esc` 关闭。
- 无结果时给「没有找到，试试：json / 截图 / 压缩」这类引导，不是空白。
- 1000 条候选内响应 ≤ 50ms：索引在启动时建一次，不在每次输入时全扫。

### 7.3 快捷键（全局一致，冲突即缺陷）

| 键 | 行为 |
|---|---|
| `Cmd+K` | 命令面板 |
| `Cmd+W` | 关闭当前工具，回首页 |
| `Cmd+,` | 设置 |
| `Cmd+Shift+V` | 剪贴板历史（开启时，全局注册） |
| `Esc` | 取消当前弹窗/长任务确认 |
| `Cmd+Enter` | 执行主操作（如「开始转换」） |
| `Space` | 图片查看器下一张（非输入态） |

> 全局热键只注册 `Cmd+Shift+V` 一个。注册失败（被别的 App 占用）时提示「快捷键冲突，可在设置里改」，不静默失效。

### 7.4 三态与文案

| 态 | 要求 | 反例 |
|---|---|---|
| 空态 | 说「下一步做什么」，给一个示例或按钮 | 一片空白 |
| 忙碌态 | 进度 + 可取消 + 已用时 | 转圈不说话 |
| 错误态 | 人话 + 原因 + 一个动作 + 可复制的详情 | `Error: 0x5` |

文案规则：正文不出现错误码（放「详情」）；不甩锅（「文件被占用」而不是「操作失败」）；给动作（「关闭 Excel 后重试」）。

### 7.5 视觉

- 一套设计 token（`ui/src/styles/tokens.css`）：间距 4 的倍数、圆角 8、语义色固定（成功/警告/危险）。
- 深色模式必须真做对比度检查（正文 ≥ 4.5:1），不许简单反色。
- 等宽字体用于 JSON/日志/哈希：`SF Mono` 优先，fallback `Menlo`。
- 图标：0.1 用一套开源线性图标占位；发布前统一替换（C6.5）。

### 7.6 键盘可达与无障碍

工具箱的用户里有相当比例是键盘党，这一节不是加分项：

| 要求 | 验收 |
|---|---|
| 所有可点区域可用 `Tab` 到达，焦点环可见 | 关闭鼠标，用 `Tab` 走完首页 → 打开工具 → 返回 |
| 焦点顺序 = 视觉顺序 | 同一页面两次 `Tab` 路径一致 |
| 图标按钮有 `title` 与 `aria-label` | 读屏能念出「清空剪贴板」 |
| 正文对比度 ≥ 4.5:1，大字 ≥ 3:1 | 深色/浅色各抽查 5 处 |
| 不依赖颜色单独传达状态 | 成功/失败除颜色外还有图标或文字 |

### 7.7 首次启动引导（90 秒内让人用上第一个工具）

| 步 | 内容 | 可跳过 |
|---|---|---|
| 1 | 一句话说明「这是什么，不联网」 | 否（3 秒可读完） |
| 2 | 选一个最可能用的工具（解压/看图/截图/文本 四选一） | 是 |
| 3 | 权限说明（仅在需要时弹，比如截图） | 是 |
| 4 | 直接进该工具，并给一个示例文件按钮 | — |

禁止：注册、问卷、订阅、评分请求、任何形式的「欢迎页轮播」。

---

## 8. 安全与隐私

### 8.1 威胁模型（按「用户把磁盘交给它」来写）

| # | 威胁 | 场景 | 防线 | 验收 |
|---|---|---|---|---|
| 1 | 路径穿越 | 文件名含 `../../`、绝对路径 | `SafePath::resolve` 归一后必须仍在用户选定根内 | 用例：`../../etc/passwd`、`/etc/passwd`、`a/../../b` |
| 2 | 符号链接逃逸 | 目录里软链指向 `/System` | `canonicalize` 后再判根，不跟随出界链接 | 造软链，操作被拒 |
| 3 | 解压炸弹 | 1MB 包解出 10GB | 解前统计总大小与文件数，超阈值先确认 | 造嵌套压缩，提示预估体积 |
| 4 | zip slip | 条目路径为 `../x` | 逐条目校验，拒绝绝对路径与 `..` | 造恶意 zip，中止并列出条目 |
| 5 | 命令注入 | 文件名含 `; rm -rf` | 参数数组 + 不经 shell | 带分号正常处理，不执行 |
| 6 | 剪贴板泄露 | 历史里存了密码 | 默认关闭；开启时可一键清空 | 关闭时 DB 无记录 |
| 7 | 日志泄露 | 日志写入完整路径/正文 | 只记路径哈希与错误码 | 抽查日志无文件内容 |
| 8 | 内存打爆 | 打开 2GB 文本/图片 | 超阈值拒绝；流式处理 | 500MB 文件不崩，给提示 |
| 9 | 覆盖用户文件 | 输出同名 | 默认另存；同名加后缀，不静默覆盖 | 连续两次转换，两个文件都在 |
| 10 | 插件越权 | 插件读没选的文件 | 权限白名单 + 路径显式传入 | 示例插件越界被拒 |
| 11 | 子进程卡死 | 坏文件让 ffmpeg 挂住 | 超时 + 取消 + 输出上限 + 强杀 | 喂坏文件 30s 内终止 |
| 12 | 渲染层注入 | Markdown/HTML 里带 `<script>`、`onerror` | Rust 侧 `ammonia` 白名单脱敏后再交给 WebView；CSP `script-src 'self'` | 造 XSS 样例，页面不执行脚本 |

### 8.2 隐私（写进 README 第一屏）

- 无账号、无遥测、无崩溃上报、无广告。
- 除「API 调试」（用户主动发请求）外无出站；0.1 连组件下载都不做，只给安装指引。
- 数据全在 `~/Library/Application Support/NovaKit/`；卸载说明给出完整路径。
- 文件索引只存路径与元数据，不存内容，可随时删除重建。
- `localStats` 只在本机累加，永不上传（见 5.5）。

### 8.3 危险操作（必须二次确认）

| 操作 | 确认要求 |
|---|---|
| 批量重命名 | 显示「将修改 N 个文件」+ 3 个示例 + 预览 |
| 删除 | 默认走系统废纸篓（`NSFileManager.trashItem` 语义），不直接 `remove_file` |
| 结束进程 | 进程名 + PID + 影响提示 |
| 清空剪贴板 | 说明不可恢复 |
| 覆盖文件 | 逐条列出，不提供默认勾选「全部覆盖」 |

### 8.4 诊断包脱敏规则（判定 #13 的落地）

诊断包 `diag-<ts>.zip` **逐项白名单，不许「顺便带上」**：

| 允许 | 内容 | 处理 |
|---|---|---|
| 版本与系统 | App 版本、commit、macOS 版本、芯片、locale | 原样 |
| 配置 | `settings.json` | 删 `recentFiles`、`privacy.fileIndexRoots`、`components.*.path` 中用户名段（替换为 `<home>`） |
| 日志 | 最近 200 行 + 最近 3 个 `crash-*.log` | 路径中 `$HOME` 替换为 `<home>` |
| 状态 | 门禁结果、组件检测结果 | 原样 |

禁止：文件内容、剪贴板正文、便签正文、DB 文件本体、任何含真实用户名的绝对路径。

> 这是「零遥测」的代价：既然不自动上报，就必须让用户能一键给你一份**安全**的现场。

---

## 9. 测试与门禁

### 9.1 六层测试

| 层 | 覆盖 | 跑法 | 门槛 |
|---|---|---|---|
| Rust 单元 | `core/{fs,text,proc}` 纯函数、每个工具的 `run()` | `cargo test -p novakit-core` | core 覆盖率 ≥ 80% |
| TS 单元 | 模糊搜索、格式化、组件纯逻辑 | `npm --prefix ui test`（vitest） | 关键模块 ≥ 70% |
| IPC 契约 | 命令名、参数、返回结构、错误映射 | `cargo test -p novakit-app --features test`（`tauri::test::mock_builder`）+ `check-ipc` | 每条命令都在 `ipc.json` 里 |
| 集成 | 真文件系统（`tempfile`）、真 SQLite、真 7z/ffmpeg（有则跑，无则跳过并打印原因） | `cargo test --workspace` | 10 条主路径 |
| 性能 | 冷启动、内存、包体、10MB 文本、面板响应 | `cargo xtask perf`（C7.3 产出） | 6 项在 9.6 预算内 |
| 人工 | 权限、观感、首次安装 | 附录 A | 9 张人工卡 |

> 集成测试里「组件缺失就跳过」必须**打印 `SKIP: ffmpeg not found`**，不许静默跳过——静默跳过是「假绿」的源头。

### 9.2 门禁命令（11 条，`ci` 与 `ci --strict` 是聚合器）

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm --prefix ui run typecheck && npm --prefix ui test
cargo xtask validate          # 契约：ID 唯一、依赖无环、版本三处一致
cargo xtask check-ipc         # Rust 命令 ↔ ipc.json ↔ TS 类型
cargo xtask check-ui          # 前端禁用符号
cargo xtask check-size        # 文件/函数行数、依赖数
cargo xtask check-licenses    # SPDX 头 + 许可证黑名单
cargo xtask check-skips       # #[ignore] / it.skip 计数
cargo xtask check-docs        # 13 个数字
cargo xtask ci                # 串起规则型检查，遇首个失败即停
cargo xtask ci --strict       # SKIP 也判红（发布用）
```

`ci` 是渐进式的：没实现的闸门打印 `SKIP <name> (not implemented)` 继续；`--strict` 下任何 SKIP 都失败。**M0 能绿，发布不能蒙混。**

### 9.3 静态门禁规则

| 门禁 | 规则 | 命中即红 |
|---|---|---|
| `check-ui` | `ui/src/**` 不得出现 `fetch(`、`XMLHttpRequest`、`localStorage`、`indexedDB`、`@tauri-apps/plugin-fs`、`@tauri-apps/plugin-shell` | 前端越权 |
| `check-ui` | `ui/src/**` 不得 `import` 任何 `node:*` 模块 | 构建期越界 |
| `check-ipc` | 每个 `#[tauri::command]` 必须在 `contracts/ipc.json` 有同名条目 | 契约缺失 |
| `check-ipc` | `ui/src/gen/ipc.ts` 与 `ipc.json` 一致（重新生成后 `git diff --exit-code`） | 生成物漂移 |
| `check-ipc` | 前端调用的 `invoke('cmd_x')` 必须存在于 `ipc.json` | 调了不存在的命令 |
| `check-size` | 单 `.rs` ≤ 500 行；单函数 ≤ 60 行；单 `.tsx` ≤ 300 行 | 超限即拆 |
| `check-size` | 直接依赖数：Rust ≤ 35，TS ≤ 12 | 依赖膨胀 |
| `check-licenses` | 每个源文件有 SPDX 头且与目录映射一致（1.4） | 缺头或错配 |
| `check-licenses` | `cargo deny check licenses` + npm 侧 GPL/AGPL 拦截 | 出现禁用许可 |
| `check-skips` | `#[ignore]` / `it.skip` / `describe.skip` 计数为 0 | 非 0 |
| `check-docs` | 9.5 的 13 个数字与正文、代码一致 | 差 1 即红 |
| `validate` | `tools.json` ID 唯一且点分；`errors.json` code 唯一；`exit-codes.json` 唯一；卡依赖无环；版本三处一致（Cargo / `ui/package.json` / `CHANGELOG` 首条） | 任一违反 |

### 9.4 证据分级

- **E1**：命令 + 退出码 + 关键输出行。
- **E2**：计数、JSON 报告、产物清单（`dist/` 文件 + sha256）。
- **E3**：真机手记（权限弹窗、观感、首次安装）。

「我试了」不算证据；E3 不能替代 E1。每张卡的证据追加到 `docs/EVIDENCE.md`，一行一卡。

### 9.5 文档计数守卫（13 个数字）

| # | 数字 | 当前值 | 机器来源 |
|---|---|---|---|
| 1 | 工具数 | 36 | `contracts/tools.json` 条目数 |
| 2 | 任务卡 | 61 | 10.1–10.8 表格行数（10.9 汇总不计） |
| 3 | 自动卡 | 52 | 卡片 `auto` 列 |
| 4 | 人工卡 | 9 | 10.9 行数 |
| 5 | 错误码 | 25 | `contracts/errors.json` |
| 6 | 退出码 | 4 | `contracts/exit-codes.json` |
| 7 | 外部组件 | 2 | 6.1 表行数 |
| 8 | 数据表 | 4 | 5.2 表行数 |
| 9 | 门禁项 | 11 | 9.2 的检查命令数（不含两行 `ci`） |
| 10 | 交付判定 | 14 | 0.3 表行数 |
| 11 | 生命周期阶段 | 8 | 0.1 表行数（M0–M7） |
| 12 | 威胁用例 | 12 | 8.1 表行数 |
| 13 | 许可文件 | 4 | `LICENSE`+`LICENSE-MIT`+`NOTICE`+`LICENSING.md` |

改任一数字必须同时改代码常量与正文；只改一边必红。这条防的不是错字，是「手册写 36 个工具、代码里只有 21 个」这种三个月后才发现的腐烂。

### 9.6 质量预算

| 项 | 上限 | 超出怎么办 |
|---|---|---|
| 冷启动到可交互 | 2s（Apple Silicon 1.2s） | 延迟初始化非首页模块；把组件检测挪到首次进入组件页 |
| 空闲内存 | 200MB | 查图片缓存与 WebView 是否留了大 DOM |
| 安装包 dmg | 40MB | 查 assets、调试符号（`strip`）、前端 sourcemap 是否打进包 |
| 打开 10MB 文本 | 1.5s | 流式读取 + 虚拟滚动（CodeMirror 自带） |
| 命令面板响应 | 50ms（1000 条内） | 预建索引，别每次全扫 |
| 单工具目录（core） | 6 个文件 | 拆到 `core/{fs,text}` |

> Tauri 的优势在这里体现：没有自带运行时，包体主要就是你的 Rust 代码 + 前端产物。40MB 是宽松预算；超过 60MB 一定是打包时把不该带的东西带进去了。

### 9.7 发布门禁（两条都必须过）

| 门 | 内容 | 不过怎么办 |
|---|---|---|
| 机器门 | `ci --strict` 零 SKIP 零失败；`perf` 六项在预算内 | 不许发布，修到绿 |
| 人工门 | 附录 A「v0.1/v1.0 检查单」全勾，每项有 E2/E3 证据 | 不许发布，补证据 |

> 只有机器门会漏掉「能跑但没法用」；只有人工门会漏掉「看着行但测试是 skip」。两道都过才算发布候选。

---

## 10. 任务卡

> 一次一张卡；`auto=否` 停等人；`files` 是写入白名单，多写一个文件即越界。
> 卡的形状固定：`ID / 任务 / 文件 / 验收命令 / 依赖 / auto`。

### 10.1 M0 定基（11 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C0.1 | 修 PATH、装 tauri-cli、装 pkg-config、验证 Node | 无（环境） | `cargo tauri --version`、`pkg-config --version`、`node -v` 三者成功 | — | **否** |
| C0.2 | `git init` + workspace + 手册入库 + AGENTS + 许可四件套 + 首次提交 | `Cargo.toml`、`rust-toolchain.toml`、`.gitignore`、`README.md`、`AGENTS.md`、`NovaKit-开发手册.md`、`LICENSE`、`LICENSE-MIT`、`NOTICE`、`LICENSING.md`、`crates/novakit-core/Cargo.toml`、`crates/novakit-core/src/lib.rs` | `cargo metadata --no-deps >/dev/null && git rev-parse HEAD && test -s LICENSING.md` | C0.1 | 是 |
| C0.3 | 契约四件套 + 类型化（`include_str!` + 启动时校验）+ 唯一性测试 | `contracts/tools.json`、`contracts/errors.json`、`contracts/exit-codes.json`、`contracts/ipc.json`、`crates/novakit-core/src/contracts.rs`、`crates/novakit-core/tests/contracts_test.rs` | `cargo test -p novakit-core --test contracts_test` | C0.2 | 是 |
| C0.4 | xtask 骨架 + 渐进式 `ci` + `validate` | `crates/novakit-xtask/Cargo.toml`、`crates/novakit-xtask/src/main.rs`、`crates/novakit-xtask/src/validate.rs`、`.cargo/config.toml` | `cargo xtask validate && cargo xtask ci` | C0.3 | 是 |
| C0.5 | `gen-ts` + `check-ipc` + ui 骨架（Vite 能 build） | `crates/novakit-xtask/src/gen_ts.rs`、`crates/novakit-xtask/src/check_ipc.rs`、`ui/package.json`、`ui/vite.config.ts`、`ui/tsconfig.json`、`ui/index.html`、`ui/src/main.tsx`、`ui/src/gen/ipc.ts` | `npm --prefix ui run build && cargo xtask check-ipc` | C0.4 | 是 |
| C0.6 | 设置存储：原子写、损坏降级、迁移 | `crates/novakit-core/src/settings/mod.rs`、`crates/novakit-core/src/settings/migrate.rs`、`contracts/settings.schema.json`、`crates/novakit-core/tests/settings_test.rs` | `cargo test -p novakit-core --test settings_test` | C0.2 | 是 |
| C0.7 | 路径闸门与文件服务：归一、根校验、原子写、临时目录、废纸篓 | `crates/novakit-core/src/fs/safe_path.rs`、`crates/novakit-core/src/fs/safe_fs.rs`、`crates/novakit-core/tests/safe_path_test.rs` | `cargo test -p novakit-core --test safe_path_test` | C0.2 | 是 |
| C0.8 | Runner：超时、取消、输出上限、参数数组、进度流 | `crates/novakit-core/src/proc/runner.rs`、`crates/novakit-core/tests/runner_test.rs` | `cargo test -p novakit-core --test runner_test` | C0.2 | 是 |
| C0.9 | 六条门禁：ui / size / licenses / skips / docs / manifest | `crates/novakit-xtask/src/gates/*.rs`、`deny.toml`、`tool/licenses.json` | `cargo xtask check-ui && cargo xtask check-licenses && cargo xtask check-docs` | C0.4 | 是 |
| C0.10 | 最小可运行壳：`tauri.conf.json` + capabilities 最小集 + `ping` 命令 + 空窗；`tauri build --debug` 出包 | `src-tauri/Cargo.toml`、`src-tauri/build.rs`、`src-tauri/tauri.conf.json`、`src-tauri/capabilities/default.json`、`src-tauri/src/main.rs`、`src-tauri/src/lib.rs`、`src-tauri/src/commands/ping.rs` | `cargo tauri build --debug && test -d target/debug/bundle/macos/NovaKit.app` | C0.5 | 是 |
| C0.11 | `BASELINE.md`：实测版本、默认决策、冻结表；人工签字 | `docs/BASELINE.md` | `grep -q "rustc 1.100" docs/BASELINE.md && grep -q SIGNED-BY docs/BASELINE.md` | C0.10 | **否** |

> C0.1 参考命令（本机实测：rustc 在 `~/.cargo/bin` 但登录 shell 里不可见；`pkg-config` 未装）：
>
> ```bash
> echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc
> cargo install tauri-cli --version "^2" --locked
> brew install pkg-config
> ```
>
> 三条验证：`cargo tauri --version` 出 2.x、`pkg-config --version` 有输出、`cargo build --release -p novakit-core` 通过。
> **不需要装完整 Xcode**：SDK 由命令行工具提供，`hdiutil`/`codesign`/`clang` 已在系统里。
> C0.2 的许可四件套不是「顺手加的文件」：`LICENSING.md` 的目录映射必须与 1.4 逐字一致，`check-licenses` 会比对。

### 10.2 M1 外壳（7 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C1.1 | 窗口壳：最小尺寸、居中、关闭确认、单实例、窗口状态恢复 | `src-tauri/src/window.rs`、`src-tauri/tauri.conf.json`、`ui/src/app/App.tsx`、`ui/src/app/App.css` | `cargo tauri build --debug` 后手动开两次，第二次聚焦已有窗口 | C0.10 | 是 |
| C1.2 | 侧栏 + 首页卡片 + 工具路由（`lastTool` 恢复） | `ui/src/ui/Sidebar.tsx`、`ui/src/ui/Home.tsx`、`ui/src/app/router.ts`、`crates/novakit-core/src/settings/last_tool.rs` | `npm --prefix ui test -- router` | C1.1 | 是 |
| C1.3 | 命令面板：模糊搜索、分组、键盘导航、动作项 | `ui/src/ui/Palette.tsx`、`ui/src/ui/fuzzy.ts`、`ui/src/ui/fuzzy.test.ts` | `npm --prefix ui test -- fuzzy palette` | C1.2 | 是 |
| C1.4 | 状态栏 + 全局快捷键 + 关于页（版本/提交/许可/日志/使用概览） | `ui/src/ui/StatusBar.tsx`、`ui/src/app/shortcuts.ts`、`ui/src/ui/About.tsx`、`src-tauri/src/commands/about.rs` | `npm --prefix ui test -- about` | C1.2 | 是 |
| C1.5 | IPC 层 + 错误映射 + 首个真工具（编码转换 T22） | `ui/src/app/ipc.ts`、`ui/src/app/errors.ts`、`ui/src/tools/codec/*`、`src-tauri/src/commands/text.rs`、`crates/novakit-core/src/tools/codec.rs`、`crates/novakit-core/tests/codec_test.rs` | `cargo test -p novakit-core --test codec_test && cargo xtask check-ipc` | C1.2 | 是 |
| C1.6 | 设置页：主题/强调色/限制/隐私开关，即时生效并落盘 | `ui/src/ui/Settings.tsx`、`src-tauri/src/commands/settings.rs`、`crates/novakit-core/src/settings/patch.rs` | `cargo test -p novakit-core --test settings_patch` | C0.6 | 是 |
| C1.7 | 启动自检：日志滚动、DB 打开、设置损坏降级、安全模式、localStats | `src-tauri/src/lib.rs`、`src-tauri/src/state.rs`、`crates/novakit-core/src/logging.rs`、`crates/novakit-core/tests/bootstrap_test.rs` | `cargo test -p novakit-core --test bootstrap_test` | C1.1 | 是 |

### 10.3 M2 文本类（9 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C2.1 | JSON：格式化/压缩/校验/路径查询（>50MB 走流式 + 限制） | `crates/novakit-core/src/tools/json.rs`、`crates/novakit-core/tests/json_test.rs`、`ui/src/tools/json/*` | `cargo test -p novakit-core --test json_test` | C1.5 | 是 |
| C2.2 | 正则测试 + 文本工具箱（去重/排序/统计/替换） | `crates/novakit-core/src/tools/{regex,textkit}.rs`、`crates/novakit-core/tests/{regex,textkit}_test.rs`、`ui/src/tools/{regex,textkit}/*` | `cargo test -p novakit-core --test regex_test --test textkit_test` | C1.5 | 是 |
| C2.3 | 哈希校验 + 密码生成 | `crates/novakit-core/src/tools/{hash,password}.rs`、`crates/novakit-core/tests/hash_test.rs`、`ui/src/tools/{hash,password}/*` | `cargo test -p novakit-core --test hash_test` | C0.7 | 是 |
| C2.4 | 颜色转换 + 调色板 | `crates/novakit-core/src/tools/color.rs`、`crates/novakit-core/tests/color_test.rs`、`ui/src/tools/color/*` | `cargo test -p novakit-core --test color_test` | C1.5 | 是 |
| C2.5 | 文本编辑器：编码检测、换行转换、大文件只读、保存前提示（CodeMirror） | `crates/novakit-core/src/tools/editor.rs`、`crates/novakit-core/src/text/encoding.rs`、`crates/novakit-core/tests/editor_test.rs`、`ui/src/tools/editor/*` | `cargo test -p novakit-core --test editor_test` | C0.7 | 是 |
| C2.6 | Markdown：解析（pulldown-cmark）+ 脱敏（ammonia）+ 导出 HTML | `crates/novakit-core/src/tools/markdown.rs`、`crates/novakit-core/tests/markdown_test.rs`、`ui/src/tools/markdown/*` | `cargo test -p novakit-core --test markdown_test` | C2.5 | 是 |
| C2.7 | 计算器 + 单位换算 | `crates/novakit-core/src/tools/calc.rs`、`crates/novakit-core/tests/calc_test.rs`、`ui/src/tools/calc/*` | `cargo test -p novakit-core --test calc_test` | C1.5 | 是 |
| C2.8 | 二维码生成（PNG/SVG）+ 图片识别 | `crates/novakit-core/src/tools/qr.rs`、`crates/novakit-core/tests/qr_test.rs`、`ui/src/tools/qr/*` | `cargo test -p novakit-core --test qr_test` | C1.5 | 是 |
| C2.9 | 便签/待办：本地库、标签、导出 Markdown | `crates/novakit-core/src/tools/notes.rs`、`crates/novakit-core/src/db/notes_dao.rs`、`crates/novakit-core/tests/notes_test.rs`、`ui/src/tools/notes/*` | `cargo test -p novakit-core --test notes_test` | C0.6 | 是 |

### 10.4 M3 文件类（6 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C3.1 | 批量重命名：规则引擎、预览、执行、撤销 | `crates/novakit-core/src/tools/rename.rs`、`crates/novakit-core/src/text/rename_rule.rs`、`crates/novakit-core/tests/rename_test.rs`、`ui/src/tools/rename/*` | `cargo test -p novakit-core --test rename_test` | C0.7 | 是 |
| C3.2 | 文件名索引与搜索（SQLite、可停、20 万上限） | `crates/novakit-core/src/tools/search.rs`、`crates/novakit-core/src/db/index_dao.rs`、`crates/novakit-core/tests/search_test.rs`、`ui/src/tools/search/*` | `cargo test -p novakit-core --test search_test` | C0.6 | 是 |
| C3.3 | 重复文件：大小预筛 + 流式哈希 + 清单 | `crates/novakit-core/src/tools/dupes.rs`、`crates/novakit-core/src/fs/hash_walk.rs`、`crates/novakit-core/tests/dupes_test.rs`、`ui/src/tools/dupes/*` | `cargo test -p novakit-core --test dupes_test` | C2.3 | 是 |
| C3.4 | 磁盘分析：目录占用、Top100 大文件、可取消 | `crates/novakit-core/src/tools/disk.rs`、`crates/novakit-core/src/fs/walk.rs`、`crates/novakit-core/tests/disk_test.rs`、`ui/src/tools/disk/*` | `cargo test -p novakit-core --test disk_test` | C0.7 | 是 |
| C3.5 | 文件分割/合并 + 文本比较 | `crates/novakit-core/src/tools/{split,diff}.rs`、`crates/novakit-core/tests/{split,diff}_test.rs`、`ui/src/tools/{split,diff}/*` | `cargo test -p novakit-core --test split_test --test diff_test` | C0.7 | 是 |
| C3.6 | 文件名编码修复（GBK↔UTF-8、冲突策略） | `crates/novakit-core/src/tools/fixname.rs`、`crates/novakit-core/src/text/gbk.rs`、`crates/novakit-core/tests/fixname_test.rs`、`ui/src/tools/fixname/*` | `cargo test -p novakit-core --test fixname_test` | C3.1 | 是 |

### 10.5 M4 图像 / PDF / 剪贴板（6 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C4.1 | 图片查看：缩放/旋转/切换，大图不通读（按需解码） | `crates/novakit-core/src/tools/viewer.rs`、`crates/novakit-core/src/image/loader.rs`、`crates/novakit-core/tests/viewer_test.rs`、`ui/src/tools/viewer/*` | `cargo test -p novakit-core --test viewer_test` | C0.7 | 是 |
| C4.2 | 格式转换 + 压缩（批量、保目录、显示体积差） | `crates/novakit-core/src/tools/{convert,compress}.rs`、`crates/novakit-core/tests/convert_test.rs`、`ui/src/tools/{convert,compress}/*` | `cargo test -p novakit-core --test convert_test` | C4.1 | 是 |
| C4.3 | 图片批处理：缩放/旋转/裁剪/水印/长图 | `crates/novakit-core/src/tools/batchimg.rs`、`crates/novakit-core/src/image/ops.rs`、`crates/novakit-core/tests/batchimg_test.rs`、`ui/src/tools/batchimg/*` | `cargo test -p novakit-core --test batchimg_test` | C4.2 | 是 |
| C4.4 | 截图与取色（`screencapture` + 屏幕录制权限，拒绝后降级） | `crates/novakit-core/src/tools/{capture,picker}.rs`、`crates/novakit-core/src/sys/screen.rs`、`ui/src/tools/{capture,picker}/*` | 人工：授权后截图成功；拒绝后提示去系统设置 | C1.1 | **否** |
| C4.5 | PDF 概览：文本提取、页数/尺寸/元信息、系统预览打开 | `crates/novakit-core/src/tools/pdf.rs`、`crates/novakit-core/tests/pdf_test.rs`、`ui/src/tools/pdf/*` | `cargo test -p novakit-core --test pdf_test` | C1.5 | 是 |
| C4.6 | 剪贴板历史：轮询（`arboard`）、去重、置顶、搜索、可关闭 | `crates/novakit-core/src/tools/clipboard.rs`、`crates/novakit-core/src/db/clipboard_dao.rs`、`crates/novakit-core/tests/clipboard_test.rs`、`ui/src/tools/clipboard/*` | `cargo test -p novakit-core --test clipboard_test` | C0.6 | 是 |

### 10.6 M5 媒体 / 系统 / 外部组件（6 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C5.1 | 组件管理：检测顺序、版本校验、设置记录、安装指引 | `crates/novakit-core/src/components/mod.rs`、`crates/novakit-core/tests/components_test.rs`、`src-tauri/src/commands/components.rs`、`ui/src/ui/Components.tsx` | `cargo test -p novakit-core --test components_test` | C0.6 | 是 |
| C5.2 | 音视频播放：`<video>` 本地文件、倍速、字幕、播完停止 | `ui/src/tools/player/*`、`src-tauri/src/commands/media.rs`、`crates/novakit-core/src/media/probe_local.rs` | `npm --prefix ui test -- player` | C5.1 | 是 |
| C5.3 | 转码与媒体信息：预设、进度、取消、ffprobe 结构化 | `crates/novakit-core/src/tools/{transcode,media_info}.rs`、`crates/novakit-core/src/proc/ffmpeg.rs`、`crates/novakit-core/tests/transcode_test.rs`、`ui/src/tools/transcode/*` | `cargo test -p novakit-core --test transcode_test` | C5.1 | 是 |
| C5.4 | 7z：列目录、预览、解压（zip slip/炸弹防护）、打包 | `crates/novakit-core/src/tools/archive.rs`、`crates/novakit-core/src/proc/sevenzip.rs`、`crates/novakit-core/tests/archive_test.rs`、`ui/src/tools/archive/*` | `cargo test -p novakit-core --test archive_test` | C5.1 | 是 |
| C5.5 | 系统信息 + 开机启动项 + 进程列表/结束 | `crates/novakit-core/src/tools/{sysinfo,process}.rs`、`crates/novakit-core/src/sys/launchd.rs`、`ui/src/tools/{sysinfo,process}/*` | 人工：能列出启动项并关闭一项（可恢复） | C1.1 | **否** |
| C5.6 | 简易 API 调试：GET/POST、Header、历史本地存（Rust `reqwest`，走 `net` 白名单） | `crates/novakit-core/src/tools/http.rs`、`crates/novakit-core/tests/http_test.rs`、`ui/src/tools/http/*` | `cargo test -p novakit-core --test http_test` | C1.5 | 是 |

### 10.7 M6 插件与 0.1 发布（8 张）

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C6.1 | 插件宿主：manifest 校验、握手、超时、强杀、输出上限 | `crates/novakit-core/src/plugins/{host,manifest,rpc}.rs`、`crates/novakit-core/tests/plugin_host_test.rs` | `cargo test -p novakit-core --test plugin_host_test` | C0.8 | 是 |
| C6.2 | 示例插件 + 端到端：安装、调用、越权拒绝、崩溃隔离 | `plugins/sample-hash/*`、`crates/novakit-core/tests/plugin_e2e_test.rs` | `cargo test -p novakit-core --test plugin_e2e_test` | C6.1 | 是 |
| C6.3 | README：安装/首次打开/权限/组件/卸载/FAQ/不做什么/许可 | `README.md` | `cargo xtask check-docs` | C5.4 | 是 |
| C6.4 | 发布脚本：版本一致、`tauri build`、dmg、sha256、许可四件套入包 | `crates/novakit-xtask/src/release.rs` | `cargo xtask release --dry-run` | C6.3 | 是 |
| C6.5 | 图标替换 + 深色对比度 + 空态文案统一（人审） | `src-tauri/icons/*`、`ui/src/styles/*`、`ui/src/ui/**` | 人工按附录 A 观感项打勾 | C6.4 | **否** |
| C6.6 | 干净环境全链路（新用户目录、未签名 dmg、30 分钟 10 个工具） | `docs/EVIDENCE.md` | 人工计时并记录阻断点 | C6.5 | **否** |
| C6.7 | 打 tag + 发布说明（版本、已知限制、外部组件要求、许可） | git tag、`CHANGELOG.md` | `git tag -l "v0.*"` 有值且 `check-docs` 绿 | C6.6 | 是 |
| C6.8 | 诊断包导出 + 崩溃留档（按 8.4 白名单脱敏） | `crates/novakit-core/src/diag/mod.rs`、`crates/novakit-core/tests/diag_test.rs`、`src-tauri/src/commands/diag.rs` | `cargo test -p novakit-core --test diag_test` 且导出 zip 不含禁用项 | C1.7 | 是 |

### 10.8 M7 1.0 与运营（8 张）

> 这一段是「和大厂一样」的分界：0.1 之后没人教你怎么活，这 8 张卡就是活法。

| ID | 任务 | 文件 | 验收 | 依赖 | auto |
|---|---|---|---|---|---|
| C7.1 | 缺陷分诊与热修流程（S0–S3、热修分支、回滚步骤） | `docs/TRIAGE.md`、`crates/novakit-xtask/src/hotfix.rs` | `cargo xtask hotfix --dry-run` 输出分支名与目标版本 | C6.7 | 是 |
| C7.2 | 兼容矩阵与冒烟脚本（macOS 13/14/15/26 × 芯片） | `crates/novakit-xtask/src/compat.rs`、`docs/COMPAT.md` | `cargo xtask compat --matrix` 每个组合有结论 | C6.7 | 是 |
| C7.3 | 性能基线与回归（冷启动/内存/包体/大文本/面板） | `crates/novakit-xtask/src/perf.rs`、`docs/PERF.md` | 六项出数字并写入 `docs/PERF.md` | C6.4 | 是 |
| C7.4 | 贡献者流程与 CI（CONTRIBUTING、issue/PR 模板、Actions） | `CONTRIBUTING.md`、`.github/ISSUE_TEMPLATE/*`、`.github/workflows/ci.yml` | `grep -q "xtask ci" .github/workflows/ci.yml && test -s CONTRIBUTING.md` | C7.3 | 是 |
| C7.5 | 依赖升级演练（Rust / Tauri / Node / 外部组件） | `docs/UPGRADE.md`、`crates/novakit-xtask/src/deps_report.rs` | `cargo xtask deps-report` 列出可升级与阻断项 | C7.4 | 是 |
| C7.6 | 1.0 契约冻结评审（工具 ID / 配置 / DB / 插件协议 / IPC） | `docs/FREEZE.md` | 人工：`grep -q SIGNED-BY docs/FREEZE.md` | C7.5 | **否** |
| C7.7 | 1.0.0 发布（版本、CHANGELOG、迁移说明、许可复核） | git tag、`CHANGELOG.md` | 人工：`git tag -l "v1.0.*"` 有值且 `ci --strict` 绿 | C7.6 | **否** |
| C7.8 | 90 天运营回顾（Top5 报障、崩溃数、工具使用、下版范围） | `docs/REVIEW-90D.md` | 人工：含四项数据与一个「砍/留」结论 | C7.7 | **否** |

### 10.9 人工卡汇总（9 张）

| 卡 | 为什么必须人做 | 判据 |
|---|---|---|
| C0.1 | 改 PATH、装 CLI、装 pkg-config 需要交互与系统授权 | 三条版本命令成功 |
| C0.11 | 默认决策影响后面全部卡 | `SIGNED-BY` 一行 |
| C4.4 | 屏幕录制权限弹窗只有人能点 | 授权成功 + 拒绝后提示正确 |
| C5.5 | 结束进程/关启动项影响真实系统 | 操作可恢复并记录 |
| C6.5 | 观感与文案只有人能判断 | 附录 A 观感项全勾 |
| C6.6 | 干净环境不能由开发者「顺手」完成 | 计时记录 + 阻断点清单 |
| C7.6 | 契约冻结是不可逆承诺 | `FREEZE.md` 签字 |
| C7.7 | 1.0 发布是对外承诺 | tag + 发布说明 |
| C7.8 | 砍功能/定范围是产品判断 | 四项数据 + 结论 |

### 10.10 卡表自检（改表后必跑）

1. 每卡 `files` 非空，且不指向禁止新建的目录。
2. 依赖无环，`blockedBy` 只指向更早的卡。
3. 每条验收命令都有产出者（`check-docs` 会比对 9.5 的数字）。
4. 卡数 61、自动 52、人工 9 与 9.5 一致。
5. 不出现「改造已有模块」「迁移旧版本」类卡——仓库从零开始。
6. 判定 13（诊断包）与判定 14（可交接）必须各有一张卡产出：C6.8 与 C7.4。

## 11. 本地模型执行协议

> 这一章是给 qwen3-coder-next 的「操作手册」。它不需要理解整个项目，只需要在**一张卡、一组文件、一条验收命令**的范围内写出能编译的 Rust 或 TS。
> 违反这一章的任何一条，都要当作「产出无效」处理，而不是「下次注意」。

### 11.1 能力边界

| 做得好 | 做不好 | 应对 |
|---|---|---|
| 按签名填实现（单函数 ≤60 行） | 自主划分模块边界 | 卡里给死文件与函数签名，只填函数体 |
| `serde` 结构体 + 纯函数变换 | 生命周期与借用关系复杂的抽象 | 一律用拥有所有权的 `String`/`Vec`，不设计 `&mut` 交织的 API |
| `Result<T, NkError>` + `?` 传播 | 手写 `unsafe`、手搓线程池 | 命门代码（Runner/JobQueue）由人先给模板 |
| 简单的 React 组件（受控表单 + 列表） | 复杂状态机、动画编排 | 每页一个 `useTool` hook，状态枚举写死 |
| 正则、编码、格式化、哈希 | 平台 API 直调（Objective-C / FFI） | 平台能力只走已封装的 `sys/` 模块 |
| 读编译错误做局部修复 | 连续 3 轮同类错误 | 3 轮即 BLOCKED（11.6） |

五类高频退化（第一张卡就要堵）：

| 退化 | 表现 | 约束 |
|---|---|---|
| 顺手加依赖 | 「用 xxx crate 更方便」 | `Cargo.toml` 的依赖段是白名单，新依赖走人工卡 |
| 逻辑写进 UI | 在 `.tsx` 里直接算哈希、拼路径 | 只调 `invoke`；`check-ui` 兜底 |
| 吞错误 | `let _ = ...;` / `.ok()` 收尾 | 必须 `?` 或映射成 `NkError` |
| 满地 clone | 借用不过就 `clone()` | 单个函数 `clone()` ≤ 2 次，超了说明设计错了 |
| 自造错误 | `Err(anyhow!("失败"))` | 只能用 `errors.json` 里的 code |

### 11.2 十二条硬规矩（原文写进 prompt）

1. 只写卡里 `files` 列出的文件，不新建目录、不新建文档、不改 `Cargo.toml`。
2. 不改已定义的函数签名/结构体字段；不符就停并报告。
3. 业务代码禁止 `println!`（用 `tracing`）、禁止空 `if let Err(_) =`。
4. 文件操作过 `SafeFs`；子进程过 `Runner`；不出现裸 `std::fs::remove_file`。
5. 错误只能用 `NkError::new(code)`，`code` 必须存在于 `errors.json`。
6. `crates/novakit-core` 里不许出现 `tauri::`；`ui/src` 里不许出现 `fetch(`。
7. 测试不许 `#[ignore]`、不许 `it.skip`；跑不了就让它红。
8. 不加依赖、不改 `Cargo.lock`、不改 `package-lock.json`。
9. 同名文件不覆盖，输出走「另存为 + 后缀」。
10. 不写 `unsafe`、不写 `unwrap()`（除非同函数内已判过 `is_some`）。
11. 遇到 `auto=否` 立即停，输出「需要人做什么」三行。
12. 不写解释性长文；只输出 A/B/C/D 四段（11.3）。

### 11.3 输出四段（A/B/C/D）

- **A 复述**：改什么、验收命令、失败落哪个错误码。
- **B 列符号**：用到的结构体/函数/字段 + `文件:行` 出处；签名是否与附录 D 一致。
- **C 完整文件**：每文件一个代码块，块内第一行写仓库相对路径；**不用 diff**。
- **D 命令**：命令、预期退出码、预期关键输出行。

只输出 A–D，不解释思路。**B 段是防幻觉的关键**：模型必须先证明「我要用的东西真的存在」，再动手。

### 11.4 上下文预算（32k 窗口，留 30% 给输出）

| 卡类型 | 预算 | 给什么 |
|---|---|---|
| 纯逻辑工具（C2.x） | 6k | 目标文件 + 一个同类工具实现 + 测试模板 |
| 文件类工具（C3.x） | 8k | 目标文件 + `SafeFs` 接口 + `Runner` 接口 + 用例 |
| 图像/媒体（C4/C5） | 9k | 目标文件 + `JobQueue` 契约 + 一个现有工具 |
| 前端页面（C1/C2） | 6k | 页面文件 + `ToolHost` 类型 + 一个现有页面 |
| 插件宿主（C6.1） | 10k | 目标文件 + 第 4 章 + 测试样例 |
| 门禁（C0.4/C0.9） | 8k | `xtask` 骨架 + 一条已有 gate 实现 |

**不要给**：整仓、整个 `crates/novakit-core/`、整份手册、`Cargo.lock`、`node_modules`、`target/`。

### 11.5 单卡循环

```text
1. 取依赖全绿的最小卡；auto=否 → 停
2. git status 必须干净（脏就停，不 stash、不 reset）
3. 基线：cargo xtask ci（记下红在哪，别急着改）
4. prompt = AGENTS.md + 本手册相关节 + 卡 + 上下文切片 + 附录 D 签名
5. 解析 A/B/C/D；越界文件 / 符号不存在 / 新错误码 → 重试，不落盘
6. 落盘 → 跑卡的验收命令 → 绿则补测试 → commit（一个卡一个 commit）
7. 退出 0 → 追加证据行；1 → 重试 ≤3 轮；2 → 环境未就绪，停；3 → 缺组件，不算失败
```

### 11.6 Rust 编译错误协议（只允许最小修复）

| 错误码 | 典型场景 | 允许的修法 | 禁止 |
|---|---|---|---|
| `E0308` mismatched types | `String` 传 `&str`、`u32` 传 `usize` | 加 `&`/`to_string()`，或在调用点改类型 | 到处 `as` 强转 |
| `E0277` trait bound | 缺 `Serialize`/`Send` | 给结构体加 `#[derive(...)]` | 改成 `Box<dyn Any>` 绕过 |
| `E0382` use of moved value | 用了已 move 的值 | 提前 `clone()` 或改传引用 | 全函数改 `clone()` 糊过去 |
| `E0502`/`E0499` 借用冲突 | 同时可变/不可变借用 | 缩小作用域、先取值再调用 | 改 `Rc<RefCell<_>>` |
| `E0599` no method | 方法名错/未 `use` trait | 补 `use` 或改契约里的名字 | 自己写一个同名方法 |
| `E0061` 参数个数不对 | 签名改了/记错 | 按附录 D 补参数 | 把参数改成可选 |
| `E0106` missing lifetime | 返回了借用值 | 返回拥有所有权的值 | 硬加 `'static` |
| `E0433` unresolved crate | 用了没加的依赖 | **停**：这属于「加依赖」，必须问人 | 从别处抄一个同名模块 |
| `error: linker not found` | 缺 `pkg-config`/SDK | 记退出码 `2`，环境问题，不改代码 | 改 `build.rs` 绕 |
| `tauri::command` 未注册 | 忘了进 `generate_handler!` | 补进列表 + `ipc.json` | 改命令名 |
| `npm ERR!` 依赖树冲突 | 版本区间宽 | 停，报告需要人决定 | `--force` / `--legacy-peer-deps` |

同一错误连续 3 轮修不好 → 写 `docs/BLOCKED-<card>.md`（现象、已试、怀疑点），停卡。

### 11.7 前端错误协议（TS/React）

| 现象 | 允许的修法 | 禁止 |
|---|---|---|
| `TS2304 Cannot find name` | 补 import 或确认 `gen/ipc.ts` 已重新生成 | 加 `// @ts-ignore` |
| `TS2345` 参数类型不符 | 按 `gen/ipc.ts` 的类型改 | `as any` |
| `invoke` 返回 `undefined` | 检查命令是否在 `ipc.json` 且已在 `generate_handler!` 注册 | 在前端兜底造假数据 |
| React 无限重渲染 | 检查 `useEffect` 依赖数组 | 用 `setTimeout` 掩盖 |
| 组件白屏 | 看 WebView 控制台（`tauri dev` 下可见）与 `logs/` | 加空 `catch` |

### 11.8 失败分类

| 现象 | 真因 | 动作 |
|---|---|---|
| `cargo` not found | PATH 未修 | 退出码 `2`，不改代码 |
| `cargo fetch` 超时 | 代理/镜像 | 退出码 `2`，改镜像并记 `BASELINE.md` |
| `pkg-config` 缺失 | 环境 | 退出码 `2` |
| 缺 ffmpeg/7z | 组件未装 | 退出码 `3`，给安装指引，不写假成功 |
| `tauri build` 报 codesign | 未签名 + 无证书 | 0.1 不签名，改用 `--no-bundle` 验证，打包由 release 脚本处理 |
| WebView 白屏但 `cargo test` 全绿 | 前端运行时错误 | 开 `tauri dev` 看控制台，这是 E3 证据 |
| 集成测试要 GUI | 环境限制 | 用 `mock_builder` 的 IPC 测试替代，真机走人工卡 |

### 11.9 运行状态机（自动跑批的唯一记忆）

三个文件，各司其职，都在 `docs/`：

| 文件 | 谁写 | 写什么 | 覆盖策略 |
|---|---|---|---|
| `STATE.json` | **只有 harness** | 当前阶段、当前卡、状态、门禁结论、锁 | 整体覆盖 |
| `RUNLOG.md` | 每卡追加一行 | `时间 卡号 结果 耗时 commit` | 追加，不改旧行 |
| `HANDOFF.md` | 每卡覆盖 | 当前卡、下一步、阻塞、环境备注（≤20 行） | 整体覆盖 |

`STATE.json` 形状（固定，不许加字段）：

```json
{
  "phase": "M3",
  "card": "C3.2",
  "status": "DOING",
  "gate": "green",
  "updatedAt": "2026-11-26T10:04:11Z",
  "lock": { "owner": "qwen3-coder-next", "pid": 48213, "expiresAt": "2026-11-26T10:34:11Z" }
}
```

`RUNLOG.md` 一行示例：

```text
2026-11-26T10:04Z C3.1 PASS 6m12s gate=green commit=9f2c1ab
```

**收尾四步（每卡结束必做，顺序不能换）**：写 `RUNLOG` → 更新 `STATE` → 覆盖 `HANDOFF` → `git commit`。
漏任何一步，下一轮就会重复做同一张卡，或者踩到「以为绿了其实没绿」。

### 11.10 串行 autopilot（一次只跑一张卡）

| 规则 | 原因 |
|---|---|
| 同时最多 1 个 `DOING` | 并发改同一批文件必然冲突，回滚成本远高于串行等待 |
| 每卡开始前 `git status` 必须干净 | 脏工作区意味着上一卡没收尾，先修再跑 |
| 锁未过期 → 直接 `exit 0` | 防止两个进程同时跑（人工误启、cron 重叠） |
| 连续 3 卡失败 → 停，写 `HANDOFF` | 连续失败通常是环境或契约问题，继续跑只会浪费 token |
| 每 5 卡跑一次 `cargo xtask ci --strict` | 单卡绿不等于整体绿（跨卡的类型漂移要靠全量抓） |
| 遇到 `auto=否` → 停 | 人工卡停下来等人，不许「我猜他想让我这么做」 |

### 11.11 何时必须停下问人（八条，命中即停）

1. 需要**新增依赖**（Rust crate 或 npm 包）。
2. 需要**改契约**（工具 ID、错误码、命令名、结构体字段）。
3. 需要**改已提交的迁移 SQL**。
4. 需要**访问网络**（除了跑 `cargo fetch` / `npm ci` 这类构建动作）。
5. **验收命令在本机跑不了**（缺 GUI、缺权限、缺组件）。
6. **同一错误连续 3 轮**。
7. **卡与手册矛盾**（比如卡要求返回 `Result<()>`，附录 D 写的是 `Result<Value>`）。
8. **触碰安全边界**（`sudo`、写 `/System`、给 WebView 加 `fs` 权限、关 CSP）。

> 第 8 条没有例外。模型「为了跑通」放宽安全边界，是这类项目最常见的事故来源。

### 11.12 单卡提示词模板（复制即用）

```text
你是 NovaKit 项目的实现者。只做一张卡。

【硬规矩】见 AGENTS.md（已附）。违反任一条即视为无效产出。

【本卡】C3.2 文件名索引与搜索
- 允许写入：crates/novakit-core/src/tools/search.rs
             crates/novakit-core/src/db/index_dao.rs
             crates/novakit-core/tests/search_test.rs
             ui/src/tools/search/SearchPage.tsx
- 验收：cargo test -p novakit-core --test search_test
- 依赖已绿：C0.6（设置）、C2.3（哈希）
- 上限：并发 1、可取消、索引上限 20 万条、查询 200ms 内返回前 200 条

【上下文】<SafeFs 接口> <Db 连接获取方式> <同类实现：tools/dupes.rs> <测试模板>

【输出】严格四段：
A 复述（改什么/验收/失败错误码）
B 符号清单（文件:行 + 签名是否与附录 D 一致）
C 完整文件（每文件一个代码块，第一行写仓库相对路径）
D 命令与预期输出
```

---

## 12. 禁区

**产品禁区**

1. 不做账号、登录、云同步、遥测、崩溃上报。
2. 不做「一键优化/清理注册表/加速开机」。
3. 不内嵌 Office 渲染、不做浏览器、不做下载器。
4. 不做移动端、不做 Web 版。
5. 不弹广告、不做「推荐安装」、不捆绑任何东西。

**技术禁区**

6. 不给 WebView 任何文件系统、shell、http 权限；CSP 不许为了让某个功能跑通而放宽 `script-src`。
7. 不以 root 运行，不请求管理员权限，不写 `/System`、`/Library` 之外（除自己的 App Support 目录）。
8. 不用裸 `std::process::Command`、不拼 shell 字符串、不 `sh -c`。
9. 不在 `crates/novakit-core` 里依赖 `tauri`；不在 `ui/src` 里 `fetch`。
10. 不自己实现加密算法（哈希用 `sha2`，随机用 `getrandom`/`rand`）。
11. 不做静默覆盖、不做静默删除（删除必须进废纸篓）。
12. 不加载未经用户确认的插件、不下载并执行任意二进制。
13. 不为了编译通过加 `#[allow(...)]`、不 `#[ignore]` 测试。
14. 不引未记录依赖、不改已提交的迁移 SQL、不改 `Cargo.lock`。
15. 不在日志里写文件内容、密码、剪贴板正文。
16. 不用 `unwrap()`/`expect()` 处理用户输入（只允许用于「不可能失败」的常量解析）。
17. 不引入 GPL/AGPL/SSPL 依赖（见 1.4）。
18. 不做自动更新、不在启动时发任何网络请求。

---

## 13. 故障排查

| 现象 | 真因 | 处理 |
|---|---|---|
| `cargo: command not found` | `~/.cargo/bin` 不在 PATH | `export PATH="$HOME/.cargo/bin:$PATH"`，写进 `~/.zshrc` |
| `cargo tauri` 不存在 | 未装 CLI | `cargo install tauri-cli --version "^2" --locked` |
| `pkg-config` 报错 / `-sys` 编译失败 | 缺 `pkg-config` | `brew install pkg-config` |
| `tauri dev` 白屏 | 前端没起 / `devUrl` 错 | 先 `npm --prefix ui run dev` 单独看页面能否打开 |
| 生产包白屏，开发正常 | 资源路径/CSP | 查 `tauri.conf.json` 的 `frontendDist` 与 `csp`；看 WebView 控制台 |
| `invoke` 报 `command not found` | 没进 `generate_handler!` 或名字不一致 | `check-ipc` 会抓；补注册并同步 `ipc.json` |
| `invoke` 报权限错误 | capabilities 缺权限 | 在 `src-tauri/capabilities/default.json` 加对应 `allow-*` |
| 打包报 `codesign` 失败 | 无证书/钥匙串 | 0.1 不签名：`tauri build --no-bundle` 验证产物，正式包由 `xtask release` 处理 |
| dmg 打不开（`hdiutil` 失败） | 已有同名挂载点 | `hdiutil detach /Volumes/NovaKit` 后重试 |
| 双击 dmg 提示「无法验证开发者」 | 未签名 + 隔离属性 | README 写「右键 → 打开」；不教用户关 Gatekeeper |
| 截图全黑 | 未授屏幕录制 | 系统设置 → 隐私与安全性 → 屏幕录制 |
| 剪贴板监听不触发 | App Nap / 轮询未开 | 设置里开启；确认轮询间隔 500ms 且未在前台阻塞 |
| 7z 解压失败 | 文件名特殊字符 / 版本旧 | 看 `Runner` 日志里的 argv；升级 7z |
| ffmpeg 卡 0% | 输出目录不可写 / 参数错 | 查输出目录权限；看 stderr 前 20 行 |
| 结束进程无反应 | 权限不足（他人进程） | 提示「系统进程不可结束」，不重试 |
| 大文件打开卡死 | 同步读取 | 走流式 + 虚拟滚动；超阈值拒绝 |
| 中文文件名乱码 | 源编码非 UTF-8 | 用 T08 修复；编辑器手动选编码 |
| `check-docs` 报差 N | 数字不同步 | 按输出逐处改，不凑总数 |
| 迁移校验红 | 改了已提交 SQL | 还原，另加新迁移 |
| 打包后工具缺失 | 注册表漏项 | 查 `tools.json` 与 `registry.rs` 是否同源 |
| 深色模式看不清 | 硬编码颜色 | 改用 `tokens.css` 的语义变量 |
| 二进制意外变大 | 调试符号/前端 map | `strip = true`、`sourcemap: false` |

---

## 14. 发布工程

### 14.1 版本与产物

| 项 | 规则 |
|---|---|
| 版本 | `Cargo.toml` 的 `[workspace.package] version`（如 `0.1.0`）**唯一源**；`tauri.conf.json` 不写 `version` |
| 一致性 | `ui/package.json` 与 `CHANGELOG.md` 首条必须同值，由 `validate` 校验 |
| 产物名 | `NovaKit-<version>.dmg`（版本不含 `+build`） |
| 构建 | `cargo tauri build` → `target/release/bundle/macos/NovaKit.app` + `dmg/NovaKit_<ver>_aarch64.dmg` |
| 校验 | `dist/SHA256SUMS`：dmg 的 sha256 + 字节数 |
| 清单 | `dist/MANIFEST.json`：version、gitCommit、builtAt、rustc、tauri、sha256、license |

### 14.2 签名与公证（0.1 的诚实做法）

- 没有 Developer ID：产物**不签名**。README 写「首次打开：右键 → 打开」，并说明为什么。
- 不引导用户 `spctl --master-disable`（降低整机安全）。
- 有账号后再开一卡：`codesign --deep --force --options runtime` + `notarytool submit` + `stapler`。在此之前不宣称「已签名」。

### 14.3 更新策略

- **不做自动更新**（会引入网络与权限，违背 0.1 边界）。
- 「关于」页显示当前版本与「检查更新」按钮：点了才打开 releases 页面（系统浏览器），不做后台请求。
- 破坏性变更（配置/DB schema）必须在发布说明里写「回滚需先恢复备份」。

### 14.4 回滚

| 对象 | 做法 | 注意 |
|---|---|---|
| 应用 | 保留上一版 dmg，覆盖安装 | 用户数据不随 App 删除 |
| 配置 | 兼容读旧 `schemaVersion`；升版前自动备份 `settings.json.bak` | 新版本写过的配置，旧版可能读不懂 |
| 数据库 | 迁移只前不后 | **回滚前先恢复 DB 备份**，否则旧版打不开 |

> 所以每次发版必须同时留：上一版 dmg + 当时的 `novakit.db` 备份说明。做不到就别发。

### 14.5 跨平台构建（只编译，不承诺）

- CI 可选：`windows-latest` / `ubuntu-latest` 各跑一次 `cargo check`，只为防「代码写死了 macOS 路径」。
- 不做签名、不做安装包、不写进「支持平台」。兼容矩阵只写 macOS。

### 14.6 分支与发布节奏

| 项 | 规则 |
|---|---|
| 主分支 | `main` 永远可发布（`ci --strict` 绿） |
| 发布分支 | 需要冻结时开 `release/0.x`，只接 S0/S1 修复 |
| 热修 | 从 tag 开 `hotfix/0.x.y`，修完打新 tag，回合 `main` |
| 节奏 | 0.1 之后每 2 周一个 0.x；1.0 前一个月只修 bug，不加功能 |
| tag | 语义化 `v0.1.0`、`v1.0.0`；tag 必须对应一个能构建的 commit |
| 变更日志 | Keep a Changelog：`Added / Changed / Fixed / Removed / Security` |

### 14.7 制品校验与供应链

| 项 | 做法 |
|---|---|
| 可复现 | `Cargo.lock` + `package-lock.json` 入库；`rust-toolchain.toml` 固定 1.100.0 |
| 清单 | `dist/MANIFEST.json` 记录 git commit、rustc/tauri 版本、构建时间 |
| 校验 | `SHA256SUMS` 与产物一起发布；README 给一行校验命令 |
| 依赖审计 | `cargo deny check`（许可证 + 安全公告）+ `npm audit --omit=dev` |
| 分发 | 只放在自有 releases 页面，不上第三方网盘、不做「高速下载器」 |
| 包内许可 | `LICENSE`、`LICENSE-MIT`、`NOTICE`、`THIRD-PARTY-LICENSES.md` 必须在 `.app/Contents/Resources/` |

---

## 15. 运营

### 15.1 缺陷分级

| 级别 | 定义 | 响应 |
|---|---|---|
| S0 | 数据损坏/误删/无法启动 | 24h 内出说明与修复版 |
| S1 | 核心工具不可用（解压、看图、截图） | 一周内 |
| S2 | 单个工具异常但有替代路径 | 下个版本 |
| S3 | 文案、观感、边缘格式 | 排期 |

### 15.2 报障取证（三样缺一不受理）

1. 版本号 + commit + macOS 版本 + 芯片（关于页一键复制）。
2. 复现步骤（点了什么、文件类型/大小）。
3. 诊断包（关于页「复制诊断包」，已按 8.4 脱敏）。

### 15.3 兼容矩阵（写进 README）

| 维度 | 承诺 | 不承诺 |
|---|---|---|
| macOS | 13+，Apple Silicon 优先 | Intel 未测（可编译，不保证） |
| 文件系统 | APFS / exFAT | 网络盘（SMB/NFS）上的批量操作 |
| 图片 | PNG/JPEG/WebP/GIF/BMP/TIFF | HEIC/AVIF（0.1 不做） |
| 视频 | 取决于 WebView 解码能力与 ffmpeg | 4K 多轨、DRM |
| 压缩 | zip / 7z / tar / gz（解） | 创建 RAR |
| 文本 | UTF-8 / UTF-16 / GBK（读） | 超 200MB 编辑 |

### 15.4 弃用策略

- 工具 ID 发布后不改；换实现就换内部，不换 ID。
- IPC 命令弃用：先标 `deprecated` 并保留一个版本，再删；前端同步。
- 配置字段弃用：保留读取一代，写回时不再写；两代后删。
- 删功能必须写进 CHANGELOG 的 `Removed`，并说明替代路径。

### 15.5 季度回归（每季一次）

1. 升级 Rust/Tauri 到当前 stable 的**下一个** minor，跑全门禁。
2. 三个系统版本（当前、-1、-2）各跑一次启动 + 解压 + 截图。
3. 外部组件升级（ffmpeg/7z）后跑对应工具。
4. 记录到 `docs/EVIDENCE.md`：版本、结果、回归人。

### 15.6 没有遥测，怎么知道质量

零遥测是产品承诺，不是「不关心质量」的借口。四个替代信号：

| 信号 | 来源 | 用途 |
|---|---|---|
| 下载数 / 版本分布 | releases 页面统计（App 内不发任何请求） | 判断「谁还停在 0.1.0」 |
| 报障 Top5 | issue 模板 + 诊断包 | 决定下个版本修什么 |
| 本地使用计数 | `settings.json.localStats`（永不上传） | 自己决定砍哪个工具 |
| 季度回归 | 15.5 | 抓「没人报但已经坏了」的静默腐烂 |

> 没有数据也能做决策，前提是**报障模板足够好**。模板差，等于自愿失明。

### 15.7 文档与社区维护

| 文档 | 更新时机 | 谁看 |
|---|---|---|
| `README.md` | 每次发版 | 用户 |
| `CHANGELOG.md` | 每个 tag | 用户、你自己 |
| `docs/TRIAGE.md` | 分诊规则变了 | 你自己 |
| `docs/COMPAT.md` | 每次兼容测试 | 报障的人 |
| `CONTRIBUTING.md` | 贡献流程变了 | 贡献者 |
| 本手册 | 契约/卡/门禁变了 | 模型、你自己 |

> 本手册的 13 个数字由 `check-docs` 守；其余文档靠「发版检查单」（附录 A）提醒。文档腐烂从来不是「忘了写」，是「没有触发点」。

---

## 16. 成熟度对照（15 维）

| # | 维度 | 0.1 达标动作 | 验收 | 阶段 |
|---|---|---|---|---|
| 1 | 边界清晰 | 36 工具 + 明确延后/禁止清单 | 0.6 与 2.2 一致 | M0 |
| 2 | 契约单一源 | 工具/错误码/退出码/IPC 各只定义一次 | `validate` 绿 | M0 |
| 3 | 路径安全 | 所有文件操作过 `SafeFs` | 12 条威胁用例全过 | M0 |
| 4 | 进程可控 | 外部程序只经 `Runner` | 超时/取消/输出上限测试 | M0 |
| 5 | 可测 | 六层测试，core 覆盖 ≥80% | `cargo test --workspace` + 覆盖率 | M3 |
| 6 | 无跳过 | `#[ignore]`/`skip` 计数 0 | `check-skips` | M0 |
| 7 | 文档不漂移 | 13 个数字自动校验 | `check-docs` | M1 |
| 8 | 失败可读 | 每个错误有 code + 人话 + 动作 | 附录 B 逐条核对 | M2 |
| 9 | 降级诚实 | 缺组件/拒权限不假装成功 | 拔组件、拒权限各走一遍 | M5 |
| 10 | 产物可验 | dmg + SHA256SUMS + MANIFEST | 附录 A | M6 |
| 11 | 可恢复 | 配置/DB 有备份、恢复演练过 | 5.4 四步各走一次 | M6 |
| 12 | 可维护 | 新增工具流程 ≤6 步且有模板 | 按附录 C 加一个示例工具 | M6 |
| 13 | 发布后运营 | 分诊、热修、兼容矩阵、性能基线 | C7.1–C7.3 全绿 + 一次模拟 S1 | M7 |
| 14 | 社区与文档 | CONTRIBUTING、issue/PR 模板、CI | C7.4 绿 | M7 |
| 15 | 许可合规 | 分层许可 + SPDX 头 + 无 GPL 依赖 | `check-licenses` 绿 + 关于页正确 | M0 |

> 15 维里，1–7、15 在 M0 就定型，8–12 随功能铺开，13–14 只在 M7 出现。**顺序不能乱**：先有边界和契约，才谈得上「运营」。

---

## 附录 A：发布检查单

**v0.1 检查单**

```text
[ ] cargo fmt --all --check 绿
[ ] cargo clippy --workspace --all-targets -- -D warnings 绿
[ ] cargo test --workspace 绿；无 #[ignore]
[ ] npm --prefix ui run typecheck && npm --prefix ui test 绿
[ ] cargo xtask ci --strict 无 SKIP
[ ] 36 个工具逐个能打开（命令面板搜一遍）
[ ] 每个工具的空态/忙碌/错误三态至少看过一次
[ ] 路径穿越、zip slip、解压炸弹、超时取消各试一次
[ ] 断网启动，T0 工具全部可用
[ ] 拒绝屏幕录制权限，截图/取色有正确提示
[ ] 卸载 7z/ffmpeg 后对应工具有安装指引
[ ] 诊断包导出一次，确认不含文件内容与真实用户名
[ ] dmg 在干净用户目录挂载、拖入 Applications、首次打开成功
[ ] 版本一致性：Cargo / ui package / CHANGELOG 三处同值
[ ] SHA256SUMS / MANIFEST.json 生成
[ ] 包内含 LICENSE、LICENSE-MIT、NOTICE、THIRD-PARTY-LICENSES
[ ] README 写明：未签名、首次打开方式、不做什么、数据路径、许可
[ ] 上一版 dmg 与 DB 备份说明保留
[ ] tag 已打，CHANGELOG 含 Added/Changed/Fixed/Removed
```

**v1.0 检查单**（在 v0.1 基础上加）

```text
[ ] 契约冻结评审签字（docs/FREEZE.md）
[ ] 兼容矩阵在三个系统版本上跑过（docs/COMPAT.md）
[ ] 性能基线六项写入 docs/PERF.md 并在预算内
[ ] 一次模拟 S1：从报障到补丁版 ≤7 天，全过程记录
[ ] CONTRIBUTING + issue/PR 模板 + Actions 可用
[ ] 依赖升级演练记录（docs/UPGRADE.md）
[ ] 关于页许可信息与实际包内文件一致
```

## 附录 B：错误码与退出码

错误码是**唯一**的错误来源（`contracts/errors.json` → `NkError`）。UI 只显示 `message`，`code` 进日志与「详情」。

| code | name | 用户看到 | 动作 |
|---|---|---|---|
| 1001 | INVALID_INPUT | 输入不合法 | 检查格式 |
| 1002 | PATH_ESCAPE | 这个路径不在允许范围内 | 重新选择 |
| 1003 | FILE_NOT_FOUND | 文件已不存在 | 刷新列表 |
| 1004 | FILE_TOO_LARGE | 文件太大，暂不支持 | 换小文件 |
| 1005 | ENCODING_UNSUPPORTED | 无法识别文件编码 | 手动选择编码 |
| 1006 | UNSUPPORTED_FORMAT | 不支持这种格式 | 查看支持列表 |
| 1007 | OVERWRITE_BLOCKED | 目标已存在 | 改文件名 |
| 2001 | PERMISSION_DENIED | 没有系统权限 | 去系统设置 |
| 2002 | SCREEN_CAPTURE_DENIED | 未授权屏幕录制 | 打开设置 |
| 2003 | CLIPBOARD_DENIED | 无法读取剪贴板 | 手动粘贴 |
| 2004 | PROCESS_CONTROL_DENIED | 该进程不允许结束 | 换一个 |
| 3001 | COMPONENT_MISSING | 缺少外部组件 | 查看安装方法 |
| 3002 | COMPONENT_VERSION_LOW | 组件版本过低 | 升级组件 |
| 3003 | COMPONENT_FAILED | 外部程序执行失败 | 看详情 |
| 3004 | JOB_TIMEOUT | 处理超时 | 重试或换文件 |
| 3005 | JOB_CANCELED | 已取消 | — |
| 4001 | STORAGE_WRITE_FAILED | 写入失败 | 检查磁盘空间 |
| 4002 | DB_MIGRATION_FAILED | 数据升级失败 | 看日志，恢复备份 |
| 4003 | SETTINGS_CORRUPT | 配置损坏，已重置 | 重新设置 |
| 5001 | PLUGIN_INVALID_MANIFEST | 插件格式不正确 | 重新安装插件 |
| 5002 | PLUGIN_PERMISSION_DENIED | 插件缺少权限 | 查看权限 |
| 5003 | PLUGIN_PROTOCOL_ERROR | 插件通信异常 | 重装/联系作者 |
| 5004 | PLUGIN_OUTPUT_TOO_LARGE | 插件输出超限 | 联系作者 |
| 5005 | PLUGIN_CRASHED | 插件已停止 | 重试 |
| 6001 | UNSUPPORTED_PLATFORM | 当前系统不支持 | — |

退出码（`cargo xtask`）：

| code | 含义 | 门禁行为 |
|---|---|---|
| 0 | 成功 | 继续 |
| 1 | 有检查项失败 | 停，输出逐项差异 |
| 2 | 环境未就绪（缺 SDK/CLI/pkg-config） | 停，不改代码 |
| 3 | 缺少外部组件 | 提示安装，不算失败 |

## 附录 C：新增一个工具的标准流程（6 步）

1. 在 `contracts/tools.json` 加一条（id/title/keywords/tier），跑 `cargo xtask validate`。
2. 在 `crates/novakit-core/src/tools/<name>.rs` 实现 `Tool` + 纯函数，并在 `tools/mod.rs` 注册。
3. 先写 `crates/novakit-core/tests/<name>_test.rs`（正常/空/非法/超限四类），再补实现。
4. 在 `contracts/ipc.json` 加命令，`cargo xtask gen-ts` 重新生成，补 `commands/<domain>.rs`（≤10 行）。
5. 建 `ui/src/tools/<name>/`：页面 + hook，注册进 `tools/index.ts`。
6. 跑 `cargo xtask ci`；更新 2.2 表格与 9.5 数字（`check-docs` 会盯着）。

## 附录 D：接口签名基线（照抄，不许改名）

```rust
// crates/novakit-core/src/error.rs
#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error("{name}: {message}")]
pub struct NkError {
    pub code: u16,
    pub name: &'static str,
    pub message: String,
    pub action: String,
}
impl NkError {
    pub fn new(code: u16) -> Self;                       // code 必须存在于 errors.json
    pub fn with_message(self, m: impl Into<String>) -> Self;
}

// crates/novakit-core/src/fs/safe_path.rs
pub struct SafePath;
impl SafePath {
    pub fn resolve(root: &Path, user_path: &str) -> Result<PathBuf, NkError>; // 越界 → 1002
    pub fn is_inside(root: &Path, resolved: &Path) -> bool;
}

// crates/novakit-core/src/fs/safe_fs.rs
pub trait SafeFs: Send + Sync {
    fn read_bytes(&self, path: &Path, max_bytes: u64) -> Result<Vec<u8>, NkError>;
    fn write_atomic(&self, path: &Path, bytes: &[u8]) -> Result<(), NkError>;
    fn ensure_temp_dir(&self, prefix: &str) -> Result<PathBuf, NkError>;
    fn move_to_trash(&self, path: &Path) -> Result<(), NkError>;
}

// crates/novakit-core/src/proc/runner.rs
pub struct ProcOutput { pub code: i32, pub stdout: String, pub stderr: String }
pub trait Runner: Send + Sync {
    fn run(&self, bin: &Path, args: &[String], timeout: Duration, max_bytes: usize)
        -> Result<ProcOutput, NkError>;
    fn run_streaming(&self, bin: &Path, args: &[String], cancel: CancellationToken,
                     on_line: &mut dyn FnMut(&str)) -> Result<ProcOutput, NkError>;
}

// crates/novakit-core/src/jobs/mod.rs
pub struct JobSpec {
    pub tool: String,
    pub title: String,
    pub total: Option<u64>,
    pub run: Box<dyn Fn(JobProgress, CancellationToken) -> Result<(), NkError> + Send + Sync>,
}
pub trait JobQueue: Send + Sync {
    fn start(&self, spec: JobSpec) -> Result<String, NkError>;   // 返回 jobId
    fn cancel(&self, job_id: &str) -> Result<(), NkError>;
}

// crates/novakit-core/src/settings/mod.rs
pub trait SettingsStore: Send + Sync {
    fn get(&self) -> AppSettings;
    fn patch(&self, f: &mut dyn FnMut(&mut AppSettings)) -> Result<(), NkError>;
}

// crates/novakit-core/src/tools/mod.rs
pub trait Tool: Send + Sync {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn keywords(&self) -> &'static [&'static str];
    fn tier(&self) -> Tier;
    fn run(&self, ctx: &ToolCtx, input: serde_json::Value) -> Result<serde_json::Value, NkError>;
}
```

命令命名规则：`cmd_<域>_<动作>`，一条命令 ≤10 行，只做「取 state → 调 core → 映射错误」。

| 命令 | 作用 |
|---|---|
| `cmd_ping` | 连通性自检（M0 用） |
| `cmd_about_info` | 版本、commit、许可证、日志路径 |
| `cmd_settings_get` / `cmd_settings_patch` | 读/改设置 |
| `cmd_tools_list` | 返回 `tools.json` 内容给前端注册路由 |
| `cmd_job_start` / `cmd_job_cancel` | 长任务 |
| `cmd_fs_pick` | 打开文件/目录选择框（dialog 插件包装） |
| `cmd_tool_run` | 统一工具入口（参数：toolId + input） |
| `cmd_diag_export` | 生成诊断包路径 |
| `cmd_component_scan` | 组件检测 |
| `cmd_plugin_install` / `cmd_plugin_run` | 插件 |

> `cmd_tool_run` 是唯一允许接受任意 `toolId` 的命令：内部按 `tools.json` 白名单分发，不许动态拼接模块名。

## 附录 E：AGENTS.md 全文（复制到仓库根）

```markdown
# AGENTS.md

你在这个仓库里只做一件事：实现当前任务卡。以下规则优先级高于任何「更方便」的做法。

## 只做
- 只修改任务卡 `files` 列出的文件。
- 只填函数体；签名以手册附录 D 为准。
- 测试先写，跑绿才算完成。

## 禁止
- 加依赖、改 Cargo.toml / package.json / Cargo.lock / package-lock.json。
- 改已提交的迁移 SQL、改契约 JSON、改工具 ID、改错误码。
- 在 crates/novakit-core 里 use tauri；在 ui/src 里 fetch / localStorage。
- unsafe、unwrap()、expect()（常量解析除外）、#[allow(...)]、#[ignore]。
- println! / console.log 作为日志；用 tracing / 结构化日志。
- 裸 std::fs 删除、裸 std::process::Command、sh -c。
- 为通过编译而放宽安全边界（CSP、capabilities、路径校验）。
- 输出 diff；必须输出完整文件。

## 停下问人（八条）
新依赖 / 改契约 / 改迁移 / 要联网 / 验收跑不了 / 同错 3 轮 / 卡与手册矛盾 / 触碰安全边界。

## 输出格式
A 复述 → B 符号清单（文件:行）→ C 完整文件（首行写路径）→ D 命令与预期输出。
不写解释性长文。
```

## 附录 F：第一天怎么开始（30 分钟）

| 分钟 | 做什么 | 完成标志 |
|---|---|---|
| 0–10 | 按 C0.1 修 PATH、装 `tauri-cli`、`pkg-config` | 三条版本命令都有输出 |
| 10–15 | 把本手册复制进仓库根，`git init`，首次提交 | `git log --oneline` 有一条 |
| 15–25 | 按 C0.2 建 workspace + 许可四件套 + AGENTS.md | `cargo metadata` 能跑 |
| 25–30 | 把 C0.1 的实测版本填进 `docs/BASELINE.md` 并签字 | `grep SIGNED-BY docs/BASELINE.md` 有输出 |

第一天不要碰任何工具实现。**M0 做不干净，后面每张卡都在还债。**

## 附录 G：一句话交接

> NovaKit 的价值不在「功能多」，而在**每个功能都小得能修、错得能懂、坏了能退**：36 个工具共用一套路径闸门和一个进程守卫，前端没有文件系统权限，不联网、不越权、不假装成功。做到这四件事，它才配被装在别人的新电脑上。


