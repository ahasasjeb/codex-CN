use codex_chatgpt::apply_command::apply_diff_from_task;
use codex_chatgpt::get_task::GetTaskResponse;
use std::path::Path;
use tempfile::TempDir;
use tokio::process::Command;

/// Creates a temporary git repository with initial commit
async fn create_temp_git_repo() -> anyhow::Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();
    let envs = vec![
        ("GIT_CONFIG_GLOBAL", "/dev/null"),
        ("GIT_CONFIG_NOSYSTEM", "1"),
    ];

    let output = Command::new("git")
        .envs(envs.clone())
        .args(["init"])
        .current_dir(repo_path)
        .output()
        .await?;

    if !output.status.success() {
            anyhow::bail!(
                "初始化 git 仓库失败: {}",
                String::from_utf8_lossy(&output.stderr)
            );
    }

    Command::new("git")
        .envs(envs.clone())
        .args(["config", "user.email", "test@example.com"])
        .current_dir(repo_path)
        .output()
        .await?;

    Command::new("git")
        .envs(envs.clone())
        .args(["config", "user.name", "Test User"])
        .current_dir(repo_path)
        .output()
        .await?;

    std::fs::write(repo_path.join("README.md"), "# Test Repo\n")?;

    Command::new("git")
        .envs(envs.clone())
        .args(["add", "README.md"])
        .current_dir(repo_path)
        .output()
        .await?;

    let output = Command::new("git")
        .envs(envs.clone())
        .args(["commit", "-m", "Initial commit"])
        .current_dir(repo_path)
        .output()
        .await?;

    if !output.status.success() {
            anyhow::bail!(
                "创建初始提交失败: {}",
                String::from_utf8_lossy(&output.stderr)
            );
    }

    Ok(temp_dir)
}

async fn mock_get_task_with_fixture() -> anyhow::Result<GetTaskResponse> {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/task_turn_fixture.json");
    let fixture_content = std::fs::read_to_string(fixture_path)?;
    let response: GetTaskResponse = serde_json::from_str(&fixture_content)?;
    Ok(response)
}

#[tokio::test]
async fn test_apply_command_creates_fibonacci_file() {
    let temp_repo = create_temp_git_repo()
        .await
        .expect("创建临时 Git 仓库失败");
    let repo_path = temp_repo.path();

    let task_response = mock_get_task_with_fixture()
        .await
            .expect("加载测试夹具失败");

    apply_diff_from_task(task_response, Some(repo_path.to_path_buf()))
        .await
        .expect("应用任务中的差异失败");

    // Assert that fibonacci.js was created in scripts/ directory
    let fibonacci_path = repo_path.join("scripts/fibonacci.js");
    assert!(fibonacci_path.exists(), "fibonacci.js was not created");

    // Verify the file contents match expected
    let contents = std::fs::read_to_string(&fibonacci_path).expect("读取 fibonacci.js 失败");
    assert!(
        contents.contains("function fibonacci(n)"),
            "fibonacci.js 不包含期望的函数"
    );
    assert!(
        contents.contains("#!/usr/bin/env node"),
            "fibonacci.js 缺少 shebang"
    );
    assert!(
        contents.contains("module.exports = fibonacci;"),
            "fibonacci.js 未导出函数"
    );

    // Verify file has correct number of lines (31 as specified in fixture)
    let line_count = contents.lines().count();
    assert_eq!(
        line_count, 31,
            "fibonacci.js 应该有 31 行，实际为 {line_count}",
    );
}

#[tokio::test]
async fn test_apply_command_with_merge_conflicts() {
    let temp_repo = create_temp_git_repo()
        .await
        .expect("创建临时 Git 仓库失败");
    let repo_path = temp_repo.path();

    // Create conflicting fibonacci.js file first
    let scripts_dir = repo_path.join("scripts");
    std::fs::create_dir_all(&scripts_dir).expect("创建 scripts 目录失败");

    let conflicting_content = r#"#!/usr/bin/env node

// This is a different fibonacci implementation
function fib(num) {
  if (num <= 1) return num;
  return fib(num - 1) + fib(num - 2);
}

console.log("Running fibonacci...");
console.log(fib(10));
"#;

    let fibonacci_path = scripts_dir.join("fibonacci.js");
    std::fs::write(&fibonacci_path, conflicting_content).expect("写入冲突文件失败");

    Command::new("git")
        .args(["add", "scripts/fibonacci.js"])
        .current_dir(repo_path)
        .output()
        .await
        .expect("添加 fibonacci.js 失败");

    Command::new("git")
        .args(["commit", "-m", "Add conflicting fibonacci implementation"])
        .current_dir(repo_path)
        .output()
        .await
        .expect("提交冲突文件失败");

    let original_dir = std::env::current_dir().expect("获取当前目录失败");
    std::env::set_current_dir(repo_path).expect("切换目录失败");
    struct DirGuard(std::path::PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }
    let _guard = DirGuard(original_dir);

    let task_response = mock_get_task_with_fixture()
        .await
        .expect("加载测试夹具失败");

    let apply_result = apply_diff_from_task(task_response, Some(repo_path.to_path_buf())).await;

    assert!(
        apply_result.is_err(),
            "预期因合并冲突而失败"
    );

    let contents = std::fs::read_to_string(&fibonacci_path).expect("Failed to read fibonacci.js");

    assert!(
        contents.contains("<<<<<<< HEAD")
            || contents.contains("=======")
            || contents.contains(">>>>>>> "),
            "fibonacci.js 应包含合并冲突标记，内容为: {contents}",
    );
}
