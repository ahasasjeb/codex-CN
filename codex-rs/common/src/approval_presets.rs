use codex_core::protocol::AskForApproval;
use codex_core::protocol::SandboxPolicy;

/// A simple preset pairing an approval policy with a sandbox policy.
#[derive(Debug, Clone)]
pub struct ApprovalPreset {
    /// Stable identifier for the preset.
    pub id: &'static str,
    /// Display label shown in UIs.
    pub label: &'static str,
    /// Short human description shown next to the label in UIs.
    pub description: &'static str,
    /// Approval policy to apply.
    pub approval: AskForApproval,
    /// Sandbox policy to apply.
    pub sandbox: SandboxPolicy,
}

/// Built-in list of approval presets that pair approval and sandbox policy.
///
/// Keep this UI-agnostic so it can be reused by both TUI and MCP server.
pub fn builtin_approval_presets() -> Vec<ApprovalPreset> {
    vec![
        ApprovalPreset {
            id: "read-only",
            label: "只读",
            description: "Codex可以读取文件并回答问题。Codex需要批准才能进行编辑、运行命令或访问网络",
            approval: AskForApproval::OnRequest,
            sandbox: SandboxPolicy::ReadOnly,
        },
        ApprovalPreset {
            id: "auto",
            label: "自动",
            description: "Codex可以读取文件、进行编辑和在工作区中运行命令。Codex需要批准才能在工作区外工作或访问网络",
            approval: AskForApproval::OnRequest,
            sandbox: SandboxPolicy::new_workspace_write_policy(),
        },
        ApprovalPreset {
            id: "full-access",
            label: "完全访问",
            description: "Codex可以读取文件、进行编辑和运行命令，并且可以访问网络，无需批准。请谨慎使用",
            approval: AskForApproval::Never,
            sandbox: SandboxPolicy::DangerFullAccess,
        },
    ]
}
