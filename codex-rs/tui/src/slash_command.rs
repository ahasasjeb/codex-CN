use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Approvals,
    Review,
    New,
    Init,
    Compact,
    Diff,
    Mention,
    Status,
    Limits,
    Mcp,
    Logout,
    Quit,
    #[cfg(debug_assertions)]
    TestApproval,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::New => "在对话中开始一个新的聊天",
            SlashCommand::Init => "为 Codex 创建包含指令的 AGENTS.md 文件",
            SlashCommand::Compact => "压缩对话以避免达到上下文限制",
            SlashCommand::Review => "审查我的更改并查找问题",
            SlashCommand::Quit => "退出 Codex",
            SlashCommand::Diff => "显示 git diff（包括未跟踪的文件）",
            SlashCommand::Mention => "引用/提及一个文件",
            SlashCommand::Status => "显示当前会话配置和令牌使用情况",
            SlashCommand::Limits => "可视化每周和每小时速率限制",
            SlashCommand::Model => "选择使用的模型和推理强度",
            SlashCommand::Approvals => "选择 Codex 在无需批准时可执行的操作",
            SlashCommand::Mcp => "列出已配置的 MCP 工具",
            SlashCommand::Logout => "登出 Codex",
            #[cfg(debug_assertions)]
            SlashCommand::TestApproval => "测试审批请求",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Model
            | SlashCommand::Approvals
            | SlashCommand::Review
            | SlashCommand::Logout => false,
            SlashCommand::Diff
            | SlashCommand::Mention
            | SlashCommand::Status
            | SlashCommand::Limits
            | SlashCommand::Mcp
            | SlashCommand::Quit => true,

            #[cfg(debug_assertions)]
            SlashCommand::TestApproval => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter().map(|c| (c.command(), c)).collect()
}
