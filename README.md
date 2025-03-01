# Cursor 机器码修改工具

一个用于修改 Cursor 编辑器机器码的命令行工具。

> ⚠️ 免责声明：本工具仅供学习和研究使用。请支持正版软件，尊重开发者劳动成果。

## 功能特点

- 自动修改 Cursor 机器码
- 自动备份原始配置
- 支持恢复备份
- 支持自定义 Cursor 安装路径
- 跨平台支持 (Windows/macOS/Linux)
- 支持管理员权限检查
- 中文友好界面

## 使用说明

### 运行要求

- 需要管理员/root权限
- Cursor 编辑器必须已安装
- 运行前请关闭 Cursor

### 基本操作

1. 修改机器码
   - 自动备份原始配置
   - 生成新的机器码
   - 可选择是否立即启动 Cursor

2. 恢复备份
   - 恢复到最近的备份状态
   
3. 修改配置
   - 查看当前配置
   - 自定义 Cursor 安装路径

### 使用方法

Windows:
```bash
cargo run --release
```

macOS/Linux:
```bash
cargo run --release
```

### 依赖要求

- Rust 1.70.0 或更高版本
- 系统要求：Windows 10+/macOS 10.15+/Linux (内核 4.19+)

### 参考

- [cursor-auto-free](https://github.com/chengazhen/cursor-auto-free) - 一个完善的自动化破解工具
- [go-cursor-help](https://github.com/yuaotian/go-cursor-help) - 相同功能的 Go 语言版本



## 安全提示

- 本工具会修改系统文件，请在使用前备份重要数据
- 不要从未知来源下载本工具
- 建议在使用前先测试备份/恢复功能

## 许可证

MIT License

详见 [LICENSE](LICCENSE)

## 贡献指南

欢迎提交 Issue 和 Pull Request。在提交 PR 前，请确保：

- 代码已经格式化 (`cargo fmt`)
- 通过所有测试 (`cargo test`)
- 提供必要的文档说明

## 免责声明

本工具仅供学习和研究使用。使用本工具可能违反 Cursor 的服务条款。使用者需自行承担使用本工具的一切风险和后果。作者不对使用本工具导致的任何损失负责。