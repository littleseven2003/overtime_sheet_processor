# overtime_sheet_processor

加班表处理工具 - 从 Excel 文件中提取指定研究室的加班数据，生成格式化结果文件。

## 功能特性

- 选择 Excel 源文件（支持 .xlsx / .xls）
- 自动识别研究室列表
- 多选研究室进行数据提取
- 生成格式化 Excel 结果文件
- 支持单元格合并
- 实时处理日志显示
- 文件拖拽支持

## 技术栈

- **框架**：Tauri 2（Rust 后端 + 系统 WebView）
- **前端**：Vue 3 + TypeScript + Vite
- **UI 组件**：Naive UI
- **Excel 处理**：calamine（读取）+ rust_xlsxwriter（写入）

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+（通过 [rustup](https://rustup.rs/) 安装）

### 开发模式

```bash
# 安装前端依赖
npm install

# 启动开发模式
npm run tauri dev
```

### 生产打包

```bash
npm run tauri build
```

打包产物位于 `src-tauri/target/release/bundle/`。

## 项目结构

```
overtime_sheet_processor/
├── docs/
│   └── Design.md          # 设计文档
├── src/                    # Vue 3 前端
│   ├── components/         # 组件
│   ├── views/              # 页面
│   ├── api/                # API 封装
│   └── types/              # 类型定义
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── commands/       # Tauri 命令
│   │   ├── excel/          # Excel 处理
│   │   └── models/         # 数据模型
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 使用说明

1. 启动应用后，点击"选择 Excel 文件"按钮
2. 选择包含加班数据的 Excel 文件
3. 在研究室列表中选择需要提取的研究室
4. 点击"开始处理"
5. 选择保存位置，等待处理完成

## 文档

详细设计见 [docs/Design.md](docs/Design.md)
