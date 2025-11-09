use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::json;

use crate::llm::ToolDefinition;
use super::executor::ShellExecutor;

#[derive(Debug, Deserialize)]
#[serde(tag = "tool", rename_all = "snake_case")]
pub enum ToolArguments {
    Bash { command: String },
    Read { path: String },
    Write { path: String, content: String },
    List { pattern: Option<String> },
}

/// Get all available tool definitions for the LLM
pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            tool_type: "function".to_string(),
            function: crate::llm::provider::FunctionDefinition {
                name: "bash".to_string(),
                description: "Execute a bash command and return the output. Use this for running shell commands, git operations, package management, etc.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The bash command to execute"
                        }
                    },
                    "required": ["command"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: crate::llm::provider::FunctionDefinition {
                name: "read".to_string(),
                description: "Read the contents of a file. Returns the file content as a string.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The path to the file to read"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: crate::llm::provider::FunctionDefinition {
                name: "write".to_string(),
                description: "Write content to a file. Creates the file if it doesn't exist, overwrites if it does.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The path to the file to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "The content to write to the file"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".to_string(),
            function: crate::llm::provider::FunctionDefinition {
                name: "list".to_string(),
                description: "List files in the current directory. Optionally filter by pattern.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Optional glob pattern to filter files (e.g., '*.rs')"
                        }
                    },
                    "required": []
                }),
            },
        },
    ]
}

/// Execute a tool call
pub fn execute_tool(
    tool_name: &str,
    arguments: &str,
    executor: &ShellExecutor,
) -> Result<String> {
    tracing::info!("Executing tool: {} with args: {}", tool_name, arguments);

    match tool_name {
        "bash" => {
            let args: serde_json::Value = serde_json::from_str(arguments)?;
            let command = args["command"]
                .as_str()
                .context("Missing 'command' argument")?;

            let result = executor.execute(command)?;

            let output = if result.success {
                format!("Exit code: {}\n\nStdout:\n{}\n\nStderr:\n{}",
                    result.exit_code,
                    result.stdout,
                    result.stderr
                )
            } else {
                format!("Command failed with exit code: {}\n\nStdout:\n{}\n\nStderr:\n{}",
                    result.exit_code,
                    result.stdout,
                    result.stderr
                )
            };

            Ok(output)
        }

        "read" => {
            let args: serde_json::Value = serde_json::from_str(arguments)?;
            let path = args["path"]
                .as_str()
                .context("Missing 'path' argument")?;

            let content = executor.read_file(path)?;
            Ok(content)
        }

        "write" => {
            let args: serde_json::Value = serde_json::from_str(arguments)?;
            let path = args["path"]
                .as_str()
                .context("Missing 'path' argument")?;
            let content = args["content"]
                .as_str()
                .context("Missing 'content' argument")?;

            executor.write_file(path, content)?;
            Ok(format!("Successfully wrote to file: {}", path))
        }

        "list" => {
            let args: serde_json::Value = serde_json::from_str(arguments)?;
            let pattern = args["pattern"].as_str();

            let files = executor.list_files(pattern)?;
            Ok(files.join("\n"))
        }

        _ => anyhow::bail!("Unknown tool: {}", tool_name),
    }
}
