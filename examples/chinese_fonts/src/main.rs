#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Chinese Font Example
//!
//! This example demonstrates the best practices for using Chinese fonts in iced applications.
//! It shows how to:
//! - Load Chinese fonts properly
//! - Handle mixed language text (English + Chinese)
//! - Use different font weights and sizes
//! - Apply proper text shaping for CJK characters

use iced::{
    Center, Element, Fill, Font, Length, Task, Theme,
    widget::{
        Column, button, column, container, row, scrollable, text, text_input,
    },
};
use std::sync::OnceLock;

static CHINESE_FONT_DATA: OnceLock<&'static [u8]> = OnceLock::new();

fn chinese_font_data() -> &'static [u8] {
    CHINESE_FONT_DATA.get_or_init(|| {
        include_bytes!("../../../graphics/fonts/SourceHanSansCN-Regular.otf")
    })
}

// Define font constants
const CHINESE_FONT: Font = Font::with_name("Source Han Sans CN");
const DEFAULT_FONT: Font = Font::DEFAULT;

// Style constants for demo sections
const DEMO_SECTION_BACKGROUND: iced::Color =
    iced::Color::from_rgb(0.95, 0.95, 0.95);
const DEMO_SECTION_BORDER_RADIUS: f32 = 8.0;
const DEMO_SECTION_PADDING: u16 = 15;

// Responsive layout constants
const MAX_CONTENT_WIDTH: f32 = 1200.0;
const NAVIGATION_HEIGHT: f32 = 80.0;
const CONTENT_PADDING: u16 = 40;

pub fn main() -> iced::Result {
    iced::application(
        ChineseFontDemo::new,
        ChineseFontDemo::update,
        ChineseFontDemo::view,
    )
    .font(chinese_font_data())
    .theme(|_| Theme::GruvboxLight)
    .run()
}

#[derive(Default)]
struct ChineseFontDemo {
    current_section: Section,
    input_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Section {
    #[default]
    BasicText,
    MixedLanguage,
    FontSizes,
    TextShaping,
    RealWorldExample,
    InteractiveTest,
}

#[derive(Debug, Clone)]
enum Message {
    SectionSelected(Section),
    InputChanged(String),
}

impl ChineseFontDemo {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SectionSelected(section) => {
                self.current_section = section;
            }
            Message::InputChanged(text) => {
                self.input_text = text;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        // Create responsive navigation bar with centered content
        let navigation = container(
            row![
                nav_button(
                    "基础文本",
                    Section::BasicText,
                    self.current_section
                ),
                nav_button(
                    "混合语言",
                    Section::MixedLanguage,
                    self.current_section
                ),
                nav_button(
                    "字体大小",
                    Section::FontSizes,
                    self.current_section
                ),
                nav_button(
                    "文本整形",
                    Section::TextShaping,
                    self.current_section
                ),
                nav_button(
                    "实际应用",
                    Section::RealWorldExample,
                    self.current_section
                ),
                nav_button(
                    "交互测试",
                    Section::InteractiveTest,
                    self.current_section
                ),
            ]
            .spacing(10),
        )
        .width(Length::Fixed(MAX_CONTENT_WIDTH.min(1000.0)))
        .padding([20, 0]);

        // Create main content with responsive width
        let content = match self.current_section {
            Section::BasicText => self.basic_text_view(),
            Section::MixedLanguage => self.mixed_language_view(),
            Section::FontSizes => self.font_sizes_view(),
            Section::TextShaping => self.text_shaping_view(),
            Section::RealWorldExample => self.real_world_example_view(),
            Section::InteractiveTest => self.interactive_test_view(),
        };

        // Create responsive content container with proper centering
        let content_container = container(
            container(scrollable(content).height(Fill))
                .width(Length::Fixed(MAX_CONTENT_WIDTH))
                .center_x(Fill)
                .padding([0, CONTENT_PADDING]),
        )
        .width(Fill)
        .center_x(Fill);

        // Main layout with centered content
        let main_layout = column![
            // Navigation section with background and centering
            container(container(navigation).center_x(Fill))
                .width(Fill)
                .height(Length::Fixed(NAVIGATION_HEIGHT))
                .style(|_theme| {
                    container::Style {
                        background: Some(iced::Background::Color(
                            iced::Color::from_rgb(0.98, 0.98, 0.98),
                        )),
                        border: iced::border::rounded(0.0),
                        ..Default::default()
                    }
                }),
            // Content section
            content_container,
        ];

        // Center the entire layout
        container(main_layout)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .into()
    }

    fn basic_text_view(&self) -> Column<Message> {
        column![
            container(section_title("基础中文文本显示"))
                .width(Fill)
                .center_x(Fill),
            demo_section(
                "默认字体 vs 中文字体对比",
                column![
                    text("使用默认字体：你好，世界！Hello World!")
                        .font(DEFAULT_FONT)
                        .size(18),
                    text("使用中文字体：你好，世界！Hello World!")
                        .font(CHINESE_FONT)
                        .size(18),
                ]
            ),
            demo_section(
                "常用中文文本",
                column![
                    text("欢迎使用 iced GUI 框架").font(CHINESE_FONT).size(20),
                    text("这是一个现代化的跨平台 GUI 库")
                        .font(CHINESE_FONT)
                        .size(16),
                    text("支持 Windows、macOS 和 Linux")
                        .font(CHINESE_FONT)
                        .size(16),
                ]
            ),
            demo_section(
                "标点符号和特殊字符",
                column![
                    text("中文标点：，。！？；：\"\"''（）【】")
                        .font(CHINESE_FONT)
                        .size(16),
                    text("数字和符号：0123456789 +-*/=<>@#$%^&")
                        .font(CHINESE_FONT)
                        .size(16),
                ]
            ),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn mixed_language_view(&self) -> Column<Message> {
        column![
            container(section_title("混合语言文本处理"))
                .width(Fill)
                .center_x(Fill),
            demo_section(
                "中英文混合",
                column![
                    text("Rust 是一种系统编程语言").font(CHINESE_FONT).size(18),
                    text("iced 是用 Rust 编写的 GUI 框架")
                        .font(CHINESE_FONT)
                        .size(18),
                    text("支持 cross-platform 跨平台开发")
                        .font(CHINESE_FONT)
                        .size(18),
                ]
            ),
            demo_section(
                "技术术语混合",
                column![
                    text("API 接口设计").font(CHINESE_FONT).size(16),
                    text("JSON 数据格式").font(CHINESE_FONT).size(16),
                    text("HTTP 请求处理").font(CHINESE_FONT).size(16),
                    text("Database 数据库连接").font(CHINESE_FONT).size(16),
                ]
            ),
            demo_section(
                "代码和注释",
                column![
                    text("// 这是一个中文注释").font(CHINESE_FONT).size(14),
                    text("fn main() { // 主函数入口 }")
                        .font(CHINESE_FONT)
                        .size(14),
                    text("let 变量名 = \"中文字符串\";")
                        .font(CHINESE_FONT)
                        .size(14),
                ]
            ),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn font_sizes_view(&self) -> Column<Message> {
        column![
            container(section_title("字体大小演示"))
                .width(Fill)
                .center_x(Fill),
            demo_section(
                "不同字体大小",
                column![
                    text("超大标题 - 32px").font(CHINESE_FONT).size(32),
                    text("大标题 - 24px").font(CHINESE_FONT).size(24),
                    text("中标题 - 20px").font(CHINESE_FONT).size(20),
                    text("正文 - 16px").font(CHINESE_FONT).size(16),
                    text("小字 - 14px").font(CHINESE_FONT).size(14),
                    text("注释 - 12px").font(CHINESE_FONT).size(12),
                ]
            ),
            demo_section(
                "层次结构示例",
                column![
                    text("第一章：Rust 编程语言").font(CHINESE_FONT).size(24),
                    text("1.1 什么是 Rust").font(CHINESE_FONT).size(20),
                    text("Rust 是一种系统级编程语言，专注于安全、速度和并发。")
                        .font(CHINESE_FONT)
                        .size(16),
                    text("注：本章节介绍 Rust 的基本概念")
                        .font(CHINESE_FONT)
                        .size(12),
                ]
            ),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn text_shaping_view(&self) -> Column<Message> {
        column![
            container(section_title("文本整形和高级特性"))
                .width(Fill)
                .center_x(Fill),
            demo_section(
                "复杂中文字符",
                column![
                    text("繁体字：繁體中文顯示測試").font(CHINESE_FONT).size(18),
                    text("生僻字：龘靐齉爩麤鱻").font(CHINESE_FONT).size(18),
                    text("多音字：重庆重量，银行行走").font(CHINESE_FONT).size(18),
                ]
            ),
            demo_section(
                "文本对齐",
                column![
                    container(text("左对齐文本").font(CHINESE_FONT).size(16))
                        .width(Fill),
                    container(text("居中对齐文本").font(CHINESE_FONT).size(16))
                        .width(Fill)
                        .center_x(Fill),
                ]
            ),
            demo_section(
                "长文本处理",
                column![
                    text("这是一段很长的中文文本，用来测试文本换行和显示效果。在实际应用中，我们经常需要处理这样的长文本内容，确保它们能够正确显示和换行。")
                        .font(CHINESE_FONT)
                        .size(16),
                ]
            ),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn real_world_example_view(&self) -> Column<Message> {
        column![
            container(section_title("实际应用示例"))
                .width(Fill)
                .center_x(Fill),
            demo_section(
                "用户界面元素",
                column![
                    text("用户登录").font(CHINESE_FONT).size(20),
                    text("用户名：").font(CHINESE_FONT).size(14),
                    text("密码：").font(CHINESE_FONT).size(14),
                    row![
                        button(text("登录").font(CHINESE_FONT)).padding(10),
                        button(text("取消").font(CHINESE_FONT)).padding(10),
                    ]
                    .spacing(10),
                ]
            ),
            demo_section(
                "菜单和导航",
                column![
                    text("主菜单").font(CHINESE_FONT).size(18),
                    text("• 文件管理").font(CHINESE_FONT).size(14),
                    text("• 编辑工具").font(CHINESE_FONT).size(14),
                    text("• 视图选项").font(CHINESE_FONT).size(14),
                    text("• 帮助文档").font(CHINESE_FONT).size(14),
                ]
            ),
            demo_section(
                "状态和消息",
                column![
                    text("✓ 文件保存成功").font(CHINESE_FONT).size(14),
                    text("⚠ 网络连接不稳定").font(CHINESE_FONT).size(14),
                    text("× 操作失败，请重试").font(CHINESE_FONT).size(14),
                    text("※ 正在处理，请稍候...").font(CHINESE_FONT).size(14),
                ]
            ),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }

    fn interactive_test_view(&self) -> Column<Message> {
        column![
            container(section_title("交互式中文输入测试"))
                .width(Fill)
                .center_x(Fill),
            demo_section(
                "实时输入测试",
                column![
                    text("在下方输入框中输入中文文本，右侧将实时显示效果：")
                        .font(CHINESE_FONT)
                        .size(16),
                    row![
                        column![
                            text("输入区域").font(CHINESE_FONT).size(14),
                            text_input("请输入中文文本...", &self.input_text)
                                .on_input(Message::InputChanged)
                                .font(CHINESE_FONT)
                                .size(16)
                                .padding(10),
                            text("支持功能：").font(CHINESE_FONT).size(12),
                            text("• 中文输入法").font(CHINESE_FONT).size(12),
                            text("• 实时预览").font(CHINESE_FONT).size(12),
                            text("• 字体渲染").font(CHINESE_FONT).size(12),
                        ]
                        .spacing(10)
                        .width(Fill),
                        column![
                            text("显示效果").font(CHINESE_FONT).size(14),
                            container(if self.input_text.is_empty() {
                                text("输入内容将在此显示...")
                                    .font(CHINESE_FONT)
                                    .size(16)
                                    .color([0.6, 0.6, 0.6])
                            } else {
                                text(&self.input_text)
                                    .font(CHINESE_FONT)
                                    .size(16)
                            })
                            .padding(15)
                            .width(Fill)
                            .height(100)
                            .style(|_theme| {
                                container::Style {
                                    background: Some(iced::Background::Color(
                                        iced::Color::from_rgb(0.98, 0.98, 0.98),
                                    )),
                                    border: iced::border::rounded(5.0),
                                    ..Default::default()
                                }
                            }),
                            text("字符统计：").font(CHINESE_FONT).size(12),
                            text(format!(
                                "字符数: {}",
                                self.input_text.chars().count()
                            ))
                            .font(CHINESE_FONT)
                            .size(12),
                            text(format!("字节数: {}", self.input_text.len()))
                                .font(CHINESE_FONT)
                                .size(12),
                        ]
                        .spacing(10)
                        .width(Fill),
                    ]
                    .spacing(20),
                ]
            ),
            demo_section(
                "多种字体大小预览",
                if self.input_text.is_empty() {
                    column![
                        text("输入文本后将显示不同字体大小的预览效果")
                            .font(CHINESE_FONT)
                            .size(14)
                            .color([0.6, 0.6, 0.6])
                    ]
                } else {
                    column![
                        text(format!("大标题 (24px): {}", &self.input_text))
                            .font(CHINESE_FONT)
                            .size(24),
                        text(format!("正文 (16px): {}", &self.input_text))
                            .font(CHINESE_FONT)
                            .size(16),
                        text(format!("小字 (12px): {}", &self.input_text))
                            .font(CHINESE_FONT)
                            .size(12),
                    ]
                    .spacing(10)
                }
            ),
            demo_section(
                "字体对比测试",
                if self.input_text.is_empty() {
                    column![
                        text("输入文本后将显示默认字体与中文字体的对比效果")
                            .font(CHINESE_FONT)
                            .size(14)
                            .color([0.6, 0.6, 0.6])
                    ]
                } else {
                    column![
                        text(format!("默认字体: {}", &self.input_text))
                            .font(DEFAULT_FONT)
                            .size(16),
                        text(format!("中文字体: {}", &self.input_text))
                            .font(CHINESE_FONT)
                            .size(16),
                    ]
                    .spacing(10)
                }
            ),
            demo_section(
                "使用建议",
                column![
                    text("测试建议：").font(CHINESE_FONT).size(16),
                    text("• 尝试输入简体中文：你好世界")
                        .font(CHINESE_FONT)
                        .size(14),
                    text("• 尝试输入繁体中文：繁體中文")
                        .font(CHINESE_FONT)
                        .size(14),
                    text("• 尝试输入中英混合：Hello 世界")
                        .font(CHINESE_FONT)
                        .size(14),
                    text("• 尝试输入标点符号：，。！？")
                        .font(CHINESE_FONT)
                        .size(14),
                    text("• 尝试输入数字：2025年").font(CHINESE_FONT).size(14),
                    text("• 尝试输入长文本测试换行效果")
                        .font(CHINESE_FONT)
                        .size(14),
                ]
                .spacing(5)
            ),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Center)
    }
}

// Helper functions
fn nav_button(
    label: &str,
    section: Section,
    _current: Section,
) -> Element<Message> {
    let btn = button(text(label).font(CHINESE_FONT)).padding([8, 16]);

    btn.on_press(Message::SectionSelected(section)).into()
}

fn section_title(title: &str) -> Element<Message> {
    text(title).font(CHINESE_FONT).size(28).into()
}

fn demo_section<'a>(
    title: &'a str,
    content: Column<'a, Message>,
) -> Element<'a, Message> {
    column![
        text(title).font(CHINESE_FONT).size(18),
        container(content)
            .padding(DEMO_SECTION_PADDING)
            .style(|_theme| {
                container::Style {
                    background: Some(iced::Background::Color(
                        DEMO_SECTION_BACKGROUND,
                    )),
                    border: iced::border::rounded(DEMO_SECTION_BORDER_RADIUS),
                    ..Default::default()
                }
            })
    ]
    .spacing(10)
    .into()
}
