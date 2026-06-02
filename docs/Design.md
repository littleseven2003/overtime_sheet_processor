# 项目设计文档：加班表处理工具

## 1. 项目概述

### 1.1 项目名称
- 英文名：`overtime_sheet_processor`
- 中文名：加班表处理工具

### 1.2 项目背景
从 Excel 文件中提取指定研究室的加班数据，生成格式化结果文件。需要一个桌面应用程序，提供优美的操作界面和高效的数据处理能力。

### 1.3 项目目标
- 提供现代化、美观、易用的桌面 GUI 界面
- 高效处理 Excel 加班数据，生成格式化结果
- 跨平台支持：Windows、macOS、Linux，覆盖 x86_64 和 ARM64 架构

### 1.4 目标用户
- 需要定期处理加班数据的研究室管理人员
- 无技术背景，需要简单直观的操作界面

---

## 2. 功能范围

### 2.1 v1 必须实现
- 选择 Excel 源文件（支持 .xlsx / .xls）
- 自动识别源文件中的研究室列表
- 多选研究室进行数据提取
- 生成格式化 Excel 结果文件并指定保存位置
- 处理进度和日志实时显示
- 文件拖拽支持
- 结果数据表格预览

### 2.2 v1 暂不实现
- 批量文件处理（一次处理一个文件）
- 自定义输出格式模板
- 处理历史持久化存储

### 2.3 后续可扩展
- 批量文件处理
- 自定义研究室列表配置
- 输出模板自定义
- 处理历史记录
- 自动更新检查（Tauri Updater 插件）

---

## 3. 技术栈

### 3.1 框架
| 技术 | 版本 | 用途 |
|------|------|------|
| Tauri 2 | 2.x | 桌面应用框架（Rust 后端 + 系统 WebView） |
| Vue 3 | 3.4+ | 响应式 UI 框架 |
| TypeScript | 5.x | 类型安全 |
| Vite | 5.x | 构建工具 |

### 3.2 UI 组件
| 技术 | 用途 |
|------|------|
| Naive UI | Vue 3 组件库（DataTable、Button、Dialog 等） |
| @tauri-apps/plugin-dialog | 原生文件选择 / 保存对话框 |
| @tauri-apps/plugin-fs | 文件系统读写 |
| @tauri-apps/plugin-shell | 调用系统命令（如打开文件所在目录） |

### 3.3 后端（Rust）
| 技术 | 用途 |
|------|------|
| calamine | Excel 文件读取（支持 .xlsx / .xls / .ods） |
| rust_xlsxwriter | Excel 文件写入与样式格式化 |
| serde / serde_json | 数据序列化 |
| tauri-plugin-dialog | 原生对话框插件 |
| tauri-plugin-fs | 文件系统插件 |

---

## 4. 系统架构

```text
┌─────────────────────────────────────────────────┐
│              Tauri 桌面应用窗口                   │
│  ┌───────────────────────────────────────────┐  │
│  │         Vue 3 前端界面                     │  │
│  │   (Naive UI 组件 + 系统 WebView 渲染)      │  │
│  └──────────────────┬────────────────────────┘  │
│                     │ Tauri IPC（进程间通信）     │
│  ┌──────────────────▼────────────────────────┐  │
│  │         Rust 后端                          │  │
│  │  ┌─────────────┐  ┌──────────────────┐    │  │
│  │  │ 文件系统     │  │ Excel 处理引擎    │    │  │
│  │  │ (对话框/读写)│  │ (calamine +      │    │  │
│  │  │             │  │  rust_xlsxwriter) │    │  │
│  │  └─────────────┘  └──────────────────┘    │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

---

## 5. 目录结构

```text
overtime_sheet_processor/
├── README.md
├── package.json                    # 前端依赖
├── vite.config.ts
├── tsconfig.json
├── index.html
│
├── docs/
│   └── Design.md
│
├── src-tauri/                      # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json             # Tauri 配置
│   ├── capabilities/               # 权限配置
│   │   └── default.json
│   ├── icons/                      # 应用图标（多尺寸）
│   │   ├── 32x32.png
│   │   ├── 128x128.png
│   │   ├── icon.icns               # macOS 图标
│   │   └── icon.ico                # Windows 图标
│   └── src/
│       ├── main.rs                 # Tauri 入口
│       ├── commands/               # Tauri 命令（IPC 接口）
│       │   ├── mod.rs
│       │   ├── file.rs             # 文件操作命令
│       │   └── process.rs          # 数据处理命令
│       ├── excel/                  # Excel 处理模块
│       │   ├── mod.rs
│       │   ├── reader.rs           # Excel 读取
│       │   ├── processor.rs        # 数据处理与格式化
│       │   └── writer.rs           # Excel 写入
│       └── models/
│           └── mod.rs              # 数据结构定义
│
├── src/                            # Vue 3 前端
│   ├── main.ts
│   ├── App.vue
│   ├── api/
│   │   └── tauri.ts                # Tauri IPC 封装
│   ├── components/
│   │   ├── FileSelector.vue        # 文件选择组件
│   │   ├── LabSelector.vue         # 研究室选择组件
│   │   ├── ProcessLog.vue          # 处理日志组件
│   │   └── ResultTable.vue         # 结果预览表格
│   ├── views/
│   │   └── Home.vue                # 主页面
│   └── types/
│       └── index.ts                # TypeScript 类型定义
│
└── docs/
    └── Design.md
```

---

## 6. 核心模块设计

### 6.1 前端模块

#### FileSelector 组件
- 点击按钮打开系统原生文件选择对话框
- 支持拖拽文件到窗口
- 限制文件类型为 .xlsx / .xls
- 显示已选文件名和路径

#### LabSelector 组件
- 选择文件后自动从 Rust 后端获取研究室列表
- 复选框多选，支持全选/取消全选

#### ProcessLog 组件
- 通过 Tauri 事件系统实时接收处理日志
- 滚动显示处理进度
- 区分 info / success / error 日志级别

#### ResultTable 组件
- Naive UI DataTable 展示处理结果预览
- 显示序号、研究室、涉及人员、加班记录、建议奖励金额等列
- 支持点击按钮另存为 Excel 文件

### 6.2 Rust 后端模块

#### 文件操作模块 (commands/file.rs)
- `open_file_dialog`：打开系统文件选择对话框，返回文件路径
- `save_file_dialog`：打开系统保存文件对话框，返回保存路径
- `read_excel_meta`：读取 Excel 文件元数据（研究室列表、行数等）

#### 数据处理模块 (commands/process.rs)
- `process_overtime_data`：核心处理命令，接收源文件路径、选中研究室、输出路径
- 处理过程中通过 Tauri 事件推送日志到前端
- 返回处理结果（成功/失败、记录数）

#### Excel 读取模块 (excel/reader.rs)
- 使用 calamine 读取 .xlsx / .xls 文件
- 解析表头，定位关键列（研究室、姓名、加班记录、奖励总额）
- 按研究室筛选数据行

#### Excel 写入模块 (excel/writer.rs)
- 使用 rust_xlsxwriter 生成格式化 Excel 文件
- 表头样式：Microsoft YaHei 字体、加粗、浅蓝填充 (#9BC2E6)
- 正文样式：宋体 11pt、居中对齐、自动换行
- 边框：所有有内容的单元格添加细边框
- 合并单元格：序号列、奖惩项点列、连续相同研究室列
- 列宽和行高自适应

#### 数据处理模块 (excel/processor.rs)
- 格式化加班记录（日期提取、排序、拼接）
- 按研究室分组排序，组织输出数据
- 添加序号列和固定列

### 6.3 类型定义 (models/mod.rs)
```rust
/// 文件元数据
pub struct FileMeta {
    pub file_path: String,
    pub file_name: String,
    pub labs: Vec<String>,
    pub total_rows: usize,
}

/// 处理请求
pub struct ProcessRequest {
    pub file_path: String,
    pub selected_labs: Vec<String>,
    pub output_path: String,
}

/// 处理结果
pub struct ProcessResult {
    pub success: bool,
    pub output_path: String,
    pub total_records: usize,
    pub message: String,
}

/// 日志事件
pub struct LogEvent {
    pub level: String,   // "info" | "success" | "error"
    pub message: String,
}
```

---

## 7. Tauri IPC 命令设计

### 7.1 命令列表

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `open_file_dialog` | 无 | `Option<String>` | 打开文件选择对话框 |
| `save_file_dialog` | 无 | `Option<String>` | 打开保存文件对话框 |
| `read_file_meta` | `file_path: String` | `Result<FileMeta>` | 读取 Excel 文件元数据 |
| `process_data` | `ProcessRequest` | `Result<ProcessResult>` | 处理数据并生成结果文件 |

### 7.2 命令示例

#### open_file_dialog
```rust
#[tauri::command]
async fn open_file_dialog(app: AppHandle) -> Result<Option<String>, String> {
    let file = DialogBuilder::new()
        .add_filter("Excel 文件", &["xlsx", "xls"])
        .set_title("选择加班数据源文件")
        .blocking_pick_file();
    Ok(file.map(|f| f.path.to_string_lossy().to_string()))
}
```

#### process_data
```rust
#[tauri::command]
async fn process_data(
    app: AppHandle,
    request: ProcessRequest,
) -> Result<ProcessResult, String> {
    // 1. 发送开始日志
    app.emit("log", LogEvent {
        level: "info".into(),
        message: "开始处理文件...".into(),
    }).map_err(|e| e.to_string())?;

    // 2. 读取 Excel
    let meta = read_excel_meta(&request.file_path)
        .map_err(|e| e.to_string())?;

    // 3. 按研究室筛选并处理数据
    let records = process_records(&meta, &request.selected_labs, |msg| {
        app.emit("log", LogEvent {
            level: "info".into(),
            message: msg,
        }).ok();
    }).map_err(|e| e.to_string())?;

    // 4. 写入格式化 Excel
    write_excel(&request.output_path, &records)
        .map_err(|e| e.to_string())?;

    // 5. 发送完成日志
    app.emit("log", LogEvent {
        level: "success".into(),
        message: format!("处理完成，共 {} 条记录", records.len()),
    }).ok();

    Ok(ProcessResult {
        success: true,
        output_path: request.output_path,
        total_records: records.len(),
        message: "处理完成".into(),
    })
}
```

---

## 8. 配置

### 8.1 Tauri 配置 (tauri.conf.json)
```json
{
  "productName": "overtime_sheet_processor",
  "version": "1.0.0",
  "identifier": "com.overtime-sheet-processor.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "title": "加班表处理工具",
        "width": 900,
        "height": 650,
        "resizable": true,
        "center": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "nsis": {
        "installMode": "both"
      }
    },
    "macOS": {
      "minimumSystemVersion": "10.15"
    }
  }
}
```

### 8.2 Cargo.toml 依赖
```toml
[package]
name = "overtime-sheet-processor"
version = "1.0.0"
edition = "2021"

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
calamine = "0.26"
rust_xlsxwriter = "0.8"

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

### 8.3 前端 package.json 依赖
```json
{
  "dependencies": {
    "vue": "^3.4.0",
    "naive-ui": "^2.39.0",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0",
    "@tauri-apps/plugin-fs": "^2.0.0",
    "@tauri-apps/plugin-shell": "^2.0.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@vitejs/plugin-vue": "^5.0.0",
    "typescript": "^5.4.0",
    "vite": "^5.4.0",
    "vue-tsc": "^2.0.0"
  }
}
```

### 8.4 前端 Vite 配置 (vite.config.ts)
```typescript
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "esnext",
    minify: "esbuild",
  },
});
```

---

## 9. 跨平台打包方案

### 9.1 支持矩阵

| 平台 | 架构 | 安装包格式 | 说明 |
|------|------|-----------|------|
| Windows | x86_64 | `.msi` / `.exe` (NSIS) | 主流 Windows 安装包 |
| Windows | ARM64 | `.msi` / `.exe` (NSIS) | Surface Pro X 等 ARM 设备 |
| macOS | x86_64 (Intel) | `.dmg` / `.app` | Intel Mac |
| macOS | ARM64 (Apple Silicon) | `.dmg` / `.app` | M1/M2/M3/M4 Mac |
| macOS | Universal | `.dmg` / `.app` | 同时包含 Intel + ARM 二进制 |
| Linux | x86_64 | `.deb` / `.AppImage` / `.rpm` | 主流 Linux 发行版 |
| Linux | ARM64 | `.deb` / `.AppImage` | 树莓派等 ARM 设备 |

### 9.2 本地打包命令

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 打包当前平台（自动检测当前 OS 和架构）
npm run tauri build
```

### 9.3 交叉编译

```bash
# 安装交叉编译目标
rustup target add x86_64-pc-windows-msvc      # macOS/Linux → Windows x86_64
rustup target add aarch64-pc-windows-msvc      # macOS/Linux → Windows ARM64
rustup target add x86_64-apple-darwin           # Linux → macOS Intel
rustup target add aarch64-apple-darwin           # Linux → macOS Apple Silicon
rustup target add x86_64-unknown-linux-gnu       # macOS → Linux x86_64
rustup target add aarch64-unknown-linux-gnu      # macOS → Linux ARM64
```

> **注意**：macOS 应用必须在 macOS 上构建（Apple 公证要求），Windows 和 Linux 可交叉编译。

### 9.4 GitHub Actions CI/CD 自动打包

```yaml
# .github/workflows/build.yml
name: Build & Release

on:
  push:
    tags: ["v*"]
  workflow_dispatch:

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          # Windows
          - platform: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_suffix: win-x64
          - platform: windows-latest
            target: aarch64-pc-windows-msvc
            artifact_suffix: win-arm64

          # macOS
          - platform: macos-latest
            target: aarch64-apple-darwin
            artifact_suffix: mac-arm64
          - platform: macos-13
            target: x86_64-apple-darwin
            artifact_suffix: mac-x64

          # Linux
          - platform: ubuntu-22.04
            target: x86_64-unknown-linux-gnu
            artifact_suffix: linux-x64
          - platform: ubuntu-22.04
            target: aarch64-unknown-linux-gnu
            artifact_suffix: linux-arm64

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install Linux dependencies
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf

      - name: Install frontend dependencies
        run: npm install

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tauriScript: npx tauri
          args: --target ${{ matrix.target }}

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: release-${{ matrix.artifact_suffix }}
          path: src-tauri/target/${{ matrix.target }}/release/bundle/**/*
```

### 9.5 macOS 特殊处理

#### Universal Binary（通用二进制）
```bash
# 在 macOS 上构建同时包含 Intel + Apple Silicon 的通用二进制
npm run tauri build -- --target universal-apple-darwin
```

#### Apple 公证（Notarization）
```yaml
# 在 GitHub Actions 中配置
- name: Build and sign
  uses: tauri-apps/tauri-action@v0
  env:
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
    APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
    APPLE_ID: ${{ secrets.APPLE_ID }}
    APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
    APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
```

### 9.6 Linux 系统依赖

```bash
# Debian / Ubuntu
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf

# Fedora
sudo dnf install -y \
  webkit2gtk4.1-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch Linux
sudo pacman -S webkit2gtk-4.1 appmenu-gtk-module libappindicator-gtk3 librsvg
```

---

## 10. 开发环境搭建

### 10.1 前置条件

| 工具 | 版本要求 | 安装方式 |
|------|---------|---------|
| Node.js | 18+ | https://nodejs.org |
| Rust | 1.70+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| VS Code | 最新 | 推荐安装 Tauri 插件 |

### 10.2 一键启动开发模式

```bash
# 克隆项目
git clone <repo-url>
cd overtime_sheet_processor

# 安装前端依赖
npm install

# 启动开发模式（自动编译 Rust + 启动 Vite 热重载）
npm run tauri dev
```

---

## 11. 安全与限制

- **文件类型限制**：仅允许选择 .xlsx / .xls 文件
- **本地运行**：所有数据处理在本地完成，不上传到任何服务器
- **无网络依赖**：应用无需联网即可使用
- **最小权限**：Tauri 默认只授予必要的文件系统权限
- **错误日志**：所有处理异常在界面日志区域显示，便于排查问题

---

## 12. 开发阶段建议

### Phase 1: 项目初始化
- 使用 `create-tauri-app` 初始化项目（Vue 3 + TypeScript 模板）
- 配置 Naive UI 组件库
- 配置 Tauri 窗口和打包选项
- 验证 `npm run tauri dev` 可正常启动
- Git 初始提交

### Phase 2: 核心功能闭环
- 实现 Rust Excel 读取模块（calamine）
- 实现 Rust Excel 写入模块（rust_xlsxwriter）
- 实现数据处理逻辑（筛选、格式化、排序）
- 实现 Tauri IPC 命令（文件对话框、读取元数据、处理数据）
- 实现前端文件选择组件
- 实现前端研究室选择组件
- 实现前端处理日志组件（Tauri 事件接收）
- 实现前端结果预览表格
- 端到端功能测试

### Phase 3: 跨平台打包
- 配置应用图标（多尺寸）
- 在当前平台测试打包
- 配置 GitHub Actions CI/CD 自动构建
- 测试 Windows x86_64 安装包
- 测试 macOS ARM64 安装包
- 测试 Linux x86_64 AppImage
- 完善错误处理和用户提示
- 编写 README 使用文档

---

## 13. 给开发 Agent 的提示词

请根据本 Design.md 实现项目 `overtime_sheet_processor`。要求：
1. 严格按照文档中的 v1 功能范围开发，不要擅自添加功能
2. 优先保证项目可以通过 `npm run tauri dev` 启动开发模式
3. 前端界面使用 Naive UI 组件库，保持美观统一
4. Rust 后端使用 calamine 读取 Excel，rust_xlsxwriter 写入格式化 Excel
5. 每完成一个阶段进行一次 Git 提交
6. 不要引入设计文档之外的大型复杂依赖
7. 如发现设计不合理，先在文档中记录建议，再进行最小改动
8. 确保 Tauri 事件系统正常工作，前端能实时显示处理日志
