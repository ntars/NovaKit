# 许可分层说明（LICENSING）

本项目采用**按目录分层**的许可，而不是「二选一双许可」。目的：需要专利与商标保护的部分用 Apache-2.0，需要零摩擦复用的部分用 MIT。

## 一、分层映射

| 范围 | 许可证 | 理由 |
| --- | --- | --- |
| `src/`、`src-tauri/`、`core/` 等源代码目录 | Apache-2.0 | 含专利授权（§3）与商标保留（§6） |
| `README.md`、`README_EN.md`、`docs/`、`LICENSING.md` | MIT | 改文档不承担 NOTICE 义务 |
| `plugins/`、插件协议、示例插件 | MIT | 降低第三方写插件的门槛 |

## 二、我要用你的代码，该看哪份

| 我要做什么 | 看哪份 |
| --- | --- |
| 复制、修改、分发源码 | [LICENSE](./LICENSE)（Apache-2.0） |
| 引用、改写文档或 README | [LICENSE-MIT](./LICENSE-MIT)（MIT） |
| 按插件协议写自己的插件 | [LICENSE-MIT](./LICENSE-MIT)（MIT） |
| 使用 `NovaKit` 名称或图标 | 不在许可范围内，需另行授权 |

## 三、四条硬规则

1. 每个源文件头必须有 SPDX 标识，且与所在目录映射一致。
2. 不引入 GPL / AGPL / SSPL 依赖。
3. `NovaKit` 名称与图标不随代码授权；可 fork 代码，不可沿用名称。
4. 贡献采用 DCO（提交带 `Signed-off-by:`），不要求 CLA。

Copyright © 2026 星玖工作室 Ntarstudio.
