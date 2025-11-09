use anyhow::Result;
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::shell::{execute_tool, get_tool_definitions, ShellExecutor};

pub struct MCPServer {
    executor: ShellExecutor,
}

impl MCPServer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            executor: ShellExecutor::default(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting MCP server");

        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;

            if n == 0 {
                break; // EOF
            }

            let request: serde_json::Value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("Failed to parse request: {}", e);
                    continue;
                }
            };

            let response = self.handle_request(&request).await;
            let response_str = serde_json::to_string(&response)?;

            stdout.write_all(response_str.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }

        Ok(())
    }

    async fn handle_request(&self, request: &serde_json::Value) -> serde_json::Value {
        let method = request["method"].as_str().unwrap_or("");

        match method {
            "initialize" => {
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "aishell",
                        "version": "0.1.0"
                    }
                })
            }

            "tools/list" => {
                let tools = get_tool_definitions();
                let tool_list: Vec<_> = tools
                    .iter()
                    .map(|t| {
                        json!({
                            "name": t.function.name,
                            "description": t.function.description,
                            "inputSchema": t.function.parameters
                        })
                    })
                    .collect();

                json!({
                    "tools": tool_list
                })
            }

            "tools/call" => {
                let tool_name = request["params"]["name"].as_str().unwrap_or("");
                let arguments = request["params"]["arguments"].to_string();

                let result = match execute_tool(tool_name, &arguments, &self.executor) {
                    Ok(output) => json!({
                        "content": [{
                            "type": "text",
                            "text": output
                        }]
                    }),
                    Err(e) => json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Error: {}", e)
                        }],
                        "isError": true
                    }),
                };

                result
            }

            _ => {
                json!({
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method)
                    }
                })
            }
        }
    }
}
