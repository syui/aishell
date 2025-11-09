use anyhow::{Context, Result};
use duct::cmd;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

pub struct ShellExecutor {
    workdir: PathBuf,
    timeout: Duration,
}

impl ShellExecutor {
    pub fn new(workdir: Option<PathBuf>) -> Result<Self> {
        let workdir = workdir.unwrap_or_else(|| {
            std::env::current_dir().expect("Failed to get current directory")
        });

        Ok(Self {
            workdir,
            timeout: Duration::from_secs(300), // 5 minutes default
        })
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn execute(&self, command: &str) -> Result<ExecutionResult> {
        tracing::info!("Executing command: {}", command);

        let output = cmd!("sh", "-c", command)
            .dir(&self.workdir)
            .stdout_capture()
            .stderr_capture()
            .unchecked()
            .run()
            .context("Failed to execute command")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        let success = output.status.success();

        tracing::debug!(
            "Command result: exit_code={}, stdout_len={}, stderr_len={}",
            exit_code,
            stdout.len(),
            stderr.len()
        );

        Ok(ExecutionResult {
            stdout,
            stderr,
            exit_code,
            success,
        })
    }

    pub fn read_file(&self, path: &str) -> Result<String> {
        let full_path = self.workdir.join(path);
        std::fs::read_to_string(&full_path)
            .with_context(|| format!("Failed to read file: {}", path))
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let full_path = self.workdir.join(path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&full_path, content)
            .with_context(|| format!("Failed to write file: {}", path))
    }

    pub fn list_files(&self, pattern: Option<&str>) -> Result<Vec<String>> {
        let pattern = pattern.unwrap_or("*");

        let output = cmd!("sh", "-c", format!("ls -1 {}", pattern))
            .dir(&self.workdir)
            .stdout_capture()
            .stderr_capture()
            .unchecked()
            .run()?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();

        Ok(files)
    }
}

impl Default for ShellExecutor {
    fn default() -> Self {
        Self::new(None).expect("Failed to create default ShellExecutor")
    }
}
