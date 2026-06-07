# 计算器 (Calculator)

## 功能

一个 Tauri v2 计算器应用，按 `=` 号将表达式作为关键字打开百度搜索。

## 目录结构

```
rustc/
├── src-tauri/           # Tauri Rust 后端
│   ├── src/
│   │   ├── lib.rs       # 核心逻辑，open_baidu 命令
│   │   └── main.rs      # 入口
│   ├── capabilities/    # Tauri v2 权限配置
│   ├── gen/android/     # Android 原生工程
│   ├── tauri.conf.json  # Tauri 配置
│   └── Cargo.toml       # Rust 依赖
├── www/                 # 前端静态文件
│   └── index.html       # 计算器 UI + 交互逻辑
└── package.json
```

## 关键改动

### 2026-06-07: 修复 Android 上 = 号无法打开网页的问题

- **问题**: `open` crate 在 Android 上没有对应的系统命令，导致打开 URL 失败
- **方案**: 改用 `tauri-plugin-opener` 插件，通过 Android Intent 打开 URL
- **改动**:
  - `Cargo.toml`: 移除 `open = "5"`，添加 `tauri-plugin-opener = "2"`
  - `lib.rs`: 注册 opener 插件，`open_baidu` 命令改用 `app.opener().open_url()` 
  - 新增 `capabilities/default.json`: 配置 opener 插件权限
