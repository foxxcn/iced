# Chinese Fonts Example

这个示例演示了在 iced 应用程序中使用中文字体的最佳实践，包括响应式布局设计和交互式测试功能。

## 功能特性

- ✅ **字体加载优化**: 使用 `OnceLock` 延迟初始化，避免栈溢出
- ✅ **响应式布局**: 自适应窗口大小，内容始终居中显示
- ✅ **中英文混合**: 完美处理中英文混合文本显示
- ✅ **多种字体大小**: 展示从 12px 到 32px 的字体效果
- ✅ **交互式测试**: 实时输入测试中文字体渲染效果
- ✅ **跨平台支持**: Windows/macOS/Linux 一致的显示效果
- ✅ **开发友好**: Debug 模式显示控制台，Release 模式隐藏

## 运行示例

```bash
# 开发模式（显示控制台）
cargo run --package chinese_fonts

# 发布模式（隐藏控制台）
cargo run --package chinese_fonts --release
```

## 核心技术实现

### 1. 安全的字体加载

```rust
use std::sync::OnceLock;

// 使用静态引用避免栈溢出
static CHINESE_FONT_DATA: OnceLock<&'static [u8]> = OnceLock::new();

fn chinese_font_data() -> &'static [u8] {
    CHINESE_FONT_DATA.get_or_init(|| {
        include_bytes!("../../../graphics/fonts/SourceHanSansCN-Regular.otf")
    })
}

// 字体常量定义
const CHINESE_FONT: Font = Font::with_name("Source Han Sans CN");
```

### 2. 响应式布局设计

```rust
// 布局常量
const MAX_CONTENT_WIDTH: f32 = 1200.0;
const NAVIGATION_HEIGHT: f32 = 80.0;
const CONTENT_PADDING: u16 = 40;

// 多层居中容器
let content_container = container(
    container(scrollable(content).height(Fill))
        .width(Length::Fixed(MAX_CONTENT_WIDTH))
        .center_x(Fill)
        .padding([0, CONTENT_PADDING])
)
.width(Fill)
.center_x(Fill);
```

### 3. 条件编译优化

```rust
// Debug 模式显示控制台，Release 模式隐藏
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
```

### 4. 交互式字体测试

```rust
// 实时输入测试
text_input("请输入中文文本...", &self.input_text)
    .on_input(Message::InputChanged)
    .font(CHINESE_FONT)
    .size(16)

// 字符统计显示
text(format!("字符数: {}", self.input_text.chars().count()))
text(format!("字节数: {}", self.input_text.len()))
```

## 示例内容

### 1. 基础文本显示
- 默认字体 vs 中文字体对比
- 常用中文文本渲染
- 标点符号和特殊字符支持

### 2. 混合语言处理
- 中英文混合文本
- 技术术语混合显示
- 代码注释中的中文

### 3. 字体大小演示
- 6 种不同字体大小 (12px-32px)
- 文档层次结构示例
- 标题和正文的搭配

### 4. 文本整形特性
- 繁体字和生僻字显示
- 文本对齐方式
- 长文本换行处理

### 5. 实际应用示例
- 用户界面元素
- 菜单和导航
- 状态消息显示

### 6. 交互式测试
- 实时中文输入
- 多种字体大小预览
- 字体对比测试
- 字符统计功能

## 布局特性

### 响应式设计
- **小窗口**: 内容不被挤压，保持可读性
- **大窗口**: 内容居中显示，避免过度拉伸
- **最大化**: 限制内容宽度，提供最佳阅读体验

### 视觉层次
- **导航栏**: 固定高度，背景色区分
- **内容区域**: 居中对齐，统一间距
- **演示区块**: 圆角背景，清晰分组

## 性能优化

| 优化项目 | 实现方式 | 性能提升 |
|---------|----------|----------|
| **字体加载** | `OnceLock` 延迟初始化 | 更快的启动时间 |
| **内存安全** | 静态引用避免复制 | 防止栈溢出 |
| **布局计算** | 固定宽度容器 | 减少重排计算 |
| **控制台管理** | 条件编译 | 更好的用户体验 |

## 支持的字体格式

- ✅ **TTF** (TrueType Font)
- ✅ **OTF** (OpenType Font) - 推荐使用
- ✅ **TTC** (TrueType Collection)
- ✅ **OTC** (OpenType Collection)

## 字体选择建议

### 推荐字体: 思源黑体 (Source Han Sans)
- **优势**: 开源免费，字符覆盖全面
- **支持**: 简繁中文、日韩文字
- **格式**: OTF，更好的字形质量
- **大小**: 约 15-20MB，适合桌面应用

### 其他选择
- **微软雅黑**: Windows 系统默认
- **苹方**: macOS 系统默认
- **Noto Sans CJK**: Google 开源字体

## 最佳实践总结

### 1. 字体加载
```rust
// ✅ 推荐：延迟初始化
static FONT_DATA: OnceLock<&'static [u8]> = OnceLock::new();

// ❌ 避免：直接复制到栈
let font_data = *include_bytes!("font.otf");
```

### 2. 布局设计
```rust
// ✅ 推荐：响应式居中
container(content)
    .width(Length::Fixed(MAX_WIDTH))
    .center_x(Fill)

// ❌ 避免：固定左对齐
container(content).width(Fill)
```

### 3. 字体使用
```rust
// ✅ 推荐：显式指定中文字体
text("中文内容").font(CHINESE_FONT)

// ❌ 避免：依赖系统默认
text("中文内容") // 可能显示不正确
```

### 4. 开发体验
```rust
// ✅ 推荐：条件编译
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// ❌ 避免：总是隐藏控制台
#![windows_subsystem = "windows"]
```

## 构建和部署

### 开发构建
```bash
cargo build --package chinese_fonts
```

### 发布构建
```bash
cargo build --package chinese_fonts --release
```

### 跨平台注意事项
- **Windows**: 自动隐藏控制台窗口
- **macOS**: 支持 Retina 显示屏
- **Linux**: 兼容各种桌面环境

## 相关资源

- [iced 官方文档](https://docs.rs/iced/latest/iced/)
- [思源黑体下载](https://github.com/adobe-fonts/source-han-sans)
- [Unicode 中文字符集](https://unicode.org/charts/PDF/U4E00.pdf)
- [CSS 字体回退机制](https://developer.mozilla.org/en-US/docs/Web/CSS/font-family)

## 许可证

本示例遵循 iced 项目的许可证条款。思源黑体字体遵循 SIL Open Font License。 