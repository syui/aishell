use anyhow::{Context, Result};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use crate::llm::{create_provider, LLMProvider, Message};
use crate::shell::{execute_tool, get_tool_definitions, ShellExecutor};

pub struct Repl {
    llm: Box<dyn LLMProvider>,
    executor: ShellExecutor,
    messages: Vec<Message>,
}

impl Repl {
    pub async fn new(provider: &str, model: Option<&str>) -> Result<Self> {
        let llm = create_provider(provider, model).await?;
        let executor = ShellExecutor::default();

        let system_prompt = Message::system(
            "You are an AI assistant that helps users interact with their system through shell commands. \
            You have access to tools like bash, read, write, and list to help users accomplish their tasks. \
            When a user asks you to do something, use the appropriate tools to complete the task. \
            Always explain what you're doing and show the results to the user."
        );

        Ok(Self {
            llm,
            executor,
            messages: vec![system_prompt],
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("aishell - AI-powered shell automation");
        println!("Type 'exit' or 'quit' to exit, 'clear' to clear history\n");

        let mut rl = DefaultEditor::new()?;

        loop {
            let readline = rl.readline("aishell> ");

            match readline {
                Ok(line) => {
                    let line = line.trim();

                    if line.is_empty() {
                        continue;
                    }

                    if line == "exit" || line == "quit" {
                        println!("Goodbye!");
                        break;
                    }

                    if line == "clear" {
                        self.messages.truncate(1); // Keep only system message
                        println!("History cleared.");
                        continue;
                    }

                    rl.add_history_entry(line)?;

                    if let Err(e) = self.process_input(line).await {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn execute_once(&mut self, prompt: &str) -> Result<()> {
        self.process_input(prompt).await
    }

    async fn process_input(&mut self, input: &str) -> Result<()> {
        // Add user message
        self.messages.push(Message::user(input));

        let tools = get_tool_definitions();

        // Agent loop: keep calling LLM until it's done (no more tool calls)
        let max_iterations = 10;
        for iteration in 0..max_iterations {
            tracing::debug!("Agent loop iteration {}", iteration + 1);

            let response = self
                .llm
                .chat(self.messages.clone(), Some(tools.clone()))
                .await
                .context("Failed to get LLM response")?;

            // If there are tool calls, execute them
            if let Some(tool_calls) = response.tool_calls {
                tracing::info!("LLM requested {} tool calls", tool_calls.len());

                // Add assistant message with tool calls
                let mut assistant_msg = Message::assistant(response.content.clone());
                assistant_msg.tool_calls = Some(tool_calls.clone());
                self.messages.push(assistant_msg);

                // Execute each tool call
                for tool_call in tool_calls {
                    let tool_name = &tool_call.function.name;
                    let tool_args = &tool_call.function.arguments;

                    println!("\n[Executing tool: {}]", tool_name);

                    let result = match execute_tool(tool_name, tool_args, &self.executor) {
                        Ok(output) => output,
                        Err(e) => format!("Error executing tool: {}", e),
                    };

                    println!("{}", result);

                    // Add tool result message
                    self.messages.push(Message::tool(result, tool_call.id.clone()));
                }

                // Continue the loop to get the next response
                continue;
            }

            // No tool calls, so the LLM is done
            if !response.content.is_empty() {
                println!("\n{}\n", response.content);
                self.messages.push(Message::assistant(response.content));
            }

            break;
        }

        Ok(())
    }
}
