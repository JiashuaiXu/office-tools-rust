# Bug 记录：模块 `utils` 未找到 🐛

## 问题描述

在编译项目时遇到以下错误：

```rust
error[E0583]: file not found for module `utils`
 --> src\main.rs:9:1
  |
9 | mod utils;
  | ^^^^^^^^^^
  |
  = help: to create the module `utils`, create file "src\utils.rs" or "src\utils\mod.rs"
```

### 错误类型
- 编译错误 (E0583)
- 模块文件缺失

### 发生时间
- 2024-03-26 22:30

### 影响范围
- 项目无法编译
- 阻止了开发进度

## 原因分析

1. 在 `src/main.rs` 中声明了 `mod utils;`
2. 但是项目中没有创建对应的模块文件
3. Rust 要求声明的模块必须有对应的文件存在

## 解决方案

1. 创建 utils 模块文件：
   ```bash
   # 方案1：创建单文件模块
   touch src/utils.rs
   
   # 或方案2：创建目录模块（推荐）
   mkdir src/utils
   touch src/utils/mod.rs
   ```

2. 在 `src/utils/mod.rs` 中添加基础工具函数：
   ```rust
   use std::path::Path;

   pub fn is_python_script(path: &Path) -> bool {
       path.extension()
           .map(|ext| ext == "py")
           .unwrap_or(false)
   }

   pub fn sanitize_script_name(name: &str) -> String {
       name.replace(['/', '\\'], "_")
           .replace(['<', '>', ':', '"', '|', '?', '*'], "")
   }
   ```

## 验证步骤

1. 创建 utils 模块文件
2. 重新编译项目：`cargo build`
3. 确认编译通过，没有相关错误

## 预防措施

1. 在声明新模块时，同时创建对应的模块文件
2. 使用 IDE 插件（如 rust-analyzer）提前发现模块问题
3. 在项目模板中包含基础模块结构

## 相关文档

- [Rust 模块系统文档](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust 错误代码 E0583](https://doc.rust-lang.org/error_codes/E0583.html)

## 备注

- 这是项目初始化阶段的常见错误
- 选择了目录模块方案，便于后续添加更多工具函数
- 添加了两个基础工具函数，用于脚本名称处理和验证 