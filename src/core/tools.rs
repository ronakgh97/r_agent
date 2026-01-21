use anyhow::{Result, anyhow};
use colored::Colorize;
use forge::api::tools_registry::{Tool, ToolRegistry};
use serde_json::Value;
use std::env;
use std::process::Stdio;
#[allow(unused)]
use tokio::fs;
use tokio::process::Command;

pub fn get_default_toolset() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(LsTool);
    registry.register(ReadFileTool);
    registry.register(TimeTool);
    registry.register(RgTool);
    registry.register(PwdTool);
    registry.register(GitDiffTool);
    registry.register(GitStatusTool);
    registry.register(GitLogTool);
    registry.register(PsTool);
    registry.register(CargoCheckTool);
    // registry.register(TreeTool);
    registry.register(SafeCurlTool);
    registry
}

/// A tool to list files and directories in the current directory (cross-platform)
pub struct LsTool;

#[async_trait::async_trait]
impl Tool for LsTool {
    fn name(&self) -> &str {
        "list_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Lists files and directories in the specified path (defaults to current directory). Returns a formatted list showing names and whether each entry is a file or directory.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The directory path to list (optional, defaults to current directory)"
                        }
                    },
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            });
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = Command::new("cmd");
            c.arg("/C").arg("dir").arg(&path);
            c
        };
        #[cfg(not(target_os = "windows"))]
        let cmd = {
            let mut c = Command::new("ls");
            c.arg("-l").arg(&path);
            c
        };
        let output = cmd.stdout(Stdio::piped()).output().await;
        match output {
            Ok(out) if out.status.success() => {
                let result = String::from_utf8_lossy(&out.stdout).to_string();
                println!(
                    "{}",
                    format!(
                        "[DEBUG] LsTool executed\nListing path: {}\n[Returning] \n{}\n",
                        path, result
                    )
                    .dimmed()
                );
                Ok(result)
            }
            Err(e) => {
                // Returns Err if command fails
                let err_msg = format!("Failed to execute list command: {}", e);
                Ok(err_msg)
            }
            _ => {
                // TODO: fallback to Rust api
                let err_msg = "Failed to execute list command".to_string();
                Ok(err_msg)
            }
        }
    }
}

pub struct TreeTool;

#[async_trait::async_trait]
impl Tool for TreeTool {
    fn name(&self) -> &str {
        "tree_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Displays a tree-like structure of files and directories starting from the specified path (defaults to current directory). Useful for visualizing the hierarchy of files and folders.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The directory path to display the tree from (optional, defaults to current directory)"
                        }
                    },
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            });
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = Command::new("powershell");
            c.arg("tree").arg(&path);
            c
        };
        #[cfg(not(target_os = "windows"))]
        let mut cmd = {
            let mut c = Command::new("tree");
            c.arg(&path);
            c
        };
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await;

        match output {
            Ok(out) if out.status.success() => {
                let result = String::from_utf8_lossy(&out.stdout).to_string();
                println!(
                    "{}",
                    format!(
                        "[DEBUG] TreeTool executed\nDisplaying tree for path: {}\n[Returning] \n{}\n",
                        path, result
                    )
                    .dimmed()
                );
                Ok(result)
            }
            Ok(out) => {
                let err_msg = String::from_utf8_lossy(&out.stderr).to_string();
                Ok(format!("Tree command failed: {}", err_msg))
            }
            Err(e) => {
                let err_msg = format!(
                    "Failed to execute tree command: {}. Make sure 'tree' is installed and in your PATH.",
                    e
                );
                Ok(err_msg)
            }
        }
    }
}

pub struct ReadFileTool;

#[async_trait::async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Reads and returns the complete contents of a text file. Use this to examine source code, configuration files, documentation, or any text-based file.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read (relative or absolute)"
                        }
                    },
                    "required": ["path"]
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("missing 'path' parameter"))?;
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = Command::new("cmd");
            c.arg("/C").arg("type").arg(path);
            c
        };
        #[cfg(not(target_os = "windows"))]
        let cmd = {
            let mut c = Command::new("cat");
            c.arg(&path);
            c
        };
        let output = cmd.stdout(Stdio::piped()).output().await;
        match output {
            Ok(out) if out.status.success() => {
                let result = String::from_utf8_lossy(&out.stdout).to_string();
                println!(
                    "{}",
                    format!(
                        "[DEBUG] ReadFileTool executed\nReading file at path: {}\n[Returning] \n{}\n",
                        path, result
                    )
                        .dimmed()
                );
                Ok(result)
            }

            Err(e) => {
                let err_msg = format!("Failed to execute read file command: {}", e);
                Ok(err_msg)
            }
            _ => {
                // TODO: fallback to Rust api
                let err_msg = "Failed to execute read file command".to_string();
                Ok(err_msg)
            }
        }
    }
}

pub struct RgTool;

#[async_trait::async_trait]
impl Tool for RgTool {
    fn name(&self) -> &str {
        "ripgrep_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Search text using ripgrep",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Text or regex to search for"
                        },
                        "path": {
                            "type": "string",
                            "description": "Optional path to search in"
                        }
                    },
                    "required": ["pattern"]
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        let pattern = args["pattern"].as_str().unwrap();
        let path = args["path"].as_str();

        let mut cmd = Command::new("rg");
        cmd.arg(pattern);

        if let Some(p) = path {
            cmd.arg(p);
        }

        let output = cmd.output().await?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "{}",
                format!(
                    "[DEBUG] RgTool executed\nSearching for pattern: {}\n[Returning] \n{}\n",
                    pattern, result
                )
                .dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(err_msg)
        }
    }
}

pub struct PwdTool;

#[async_trait::async_trait]
impl Tool for PwdTool {
    fn name(&self) -> &str {
        "print_working_directory_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Prints the current working directory",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, _args: Value) -> Result<String> {
        #[cfg(target_os = "windows")]
        let output = {
            let mut c = Command::new("powershell");
            c.arg("pwd");
            c.output().await?
        };
        #[cfg(not(target_os = "windows"))]
        let cmd = {
            let mut c = Command::new("pwd");
            c.output().await?
        };
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "{}",
                format!("[DEBUG] PwdTool executed\n[Returning] \n{}\n", result).dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(err_msg)
        }
    }
}

pub struct GitDiffTool;

#[async_trait::async_trait]
impl Tool for GitDiffTool {
    fn name(&self) -> &str {
        "git_diff_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Shows git diff for the current repository",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, _args: Value) -> Result<String> {
        let output = Command::new("git").args(["diff"]).output().await?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "{}",
                format!("[DEBUG] GitDiffTool executed\n[Returning] \n{}\n", result).dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(err_msg)
        }
    }
}

pub struct GitStatusTool;

#[async_trait::async_trait]
impl Tool for GitStatusTool {
    fn name(&self) -> &str {
        "git_status_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Shows git status for the current repository",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }
    fn tool_callback(&self) -> bool {
        true
    }
    async fn execute_tool(&self, _args: Value) -> Result<String> {
        let output = Command::new("git").args(["status"]).output().await?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "{}",
                format!("[DEBUG] GitStatusTool executed\n[Returning] \n{}\n", result).dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(err_msg)
        }
    }
}

pub struct PsTool;

#[async_trait::async_trait]
impl Tool for PsTool {
    fn name(&self) -> &str {
        "process_list_tool"
    }
    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Lists running processes on the system",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }
    fn tool_callback(&self) -> bool {
        true
    }
    async fn execute_tool(&self, _args: Value) -> Result<String> {
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = Command::new("cmd");
            c.arg("/C").arg("tasklist");
            c
        };
        #[cfg(not(target_os = "windows"))]
        let cmd = {
            let mut c = Command::new("ps");
            c.arg("-aux");
            c
        };
        let output = cmd.output().await?;
        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "{}",
                format!("[DEBUG] PsTool executed\n[Returning] \n{}\n", result).dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(err_msg)
        }
    }
}

pub struct GitLogTool;

#[async_trait::async_trait]
impl Tool for GitLogTool {
    fn name(&self) -> &str {
        "git_log_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Shows git log for the current repository",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, _args: Value) -> Result<String> {
        let output = Command::new("git")
            .args(["log", "--oneline"])
            .output()
            .await?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            println!(
                "{}",
                format!("[DEBUG] GitLogTool executed\n[Returning] \n{}\n", result).dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(err_msg)
        }
    }
}

pub struct SafeCurlTool;

#[async_trait::async_trait]
impl Tool for SafeCurlTool {
    fn name(&self) -> &str {
        "safe_curl_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Performs a safe HTTP GET request to the specified URL and returns the response body. Use this tool to fetch data from web APIs or websites.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "url": {
                            "type": "string",
                            "description": "The URL to fetch data from"
                        }
                    },
                    "required": ["url"]
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, args: Value) -> Result<String> {
        let url = args["url"]
            .as_str()
            .ok_or_else(|| anyhow!("missing 'url' parameter"))?;

        let response = reqwest::get(url).await?;

        if response.status().is_success() {
            let body = response.text().await?;
            println!(
                "{}",
                format!(
                    "[DEBUG] SafeCurlTool executed\nFetching URL: {}\n[Returning] \n{}\n",
                    url, body
                )
                .dimmed()
            );
            Ok(body)
        } else {
            let err_msg = format!("Failed to fetch URL: HTTP {}", response.status());
            Ok(err_msg)
        }
    }
}

/// A tool to run 'cargo check' in the current Rust project directory
pub struct CargoCheckTool;

#[async_trait::async_trait]
impl Tool for CargoCheckTool {
    fn name(&self) -> &str {
        "cargo_check_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Runs 'cargo check' in the current Rust project directory and returns the output. Use this tool to verify that your Rust code compiles without errors.",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, _args: Value) -> Result<String> {
        let output = Command::new("cargo").arg("check").output().await?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stderr).to_string();
            println!(
                "{}",
                format!(
                    "[DEBUG] CargoCheckTool executed\n[Returning] \n{}\n",
                    result
                )
                .dimmed()
            );
            Ok(result)
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).to_string();
            println!(
                "{}",
                format!(
                    "[DEBUG] CargoCheckTool executed (with errors)\n[Returning] \n{}\n",
                    err_msg
                )
                    .dimmed()
            );
            Ok(err_msg)
        }

    }
}

/// A tool to get the current system time
pub struct TimeTool;

#[async_trait::async_trait]
impl Tool for TimeTool {
    fn name(&self) -> &str {
        "get_time_tool"
    }

    fn description(&self) -> Value {
        serde_json::json!({
            "type": "function",
            "function": {
                "name": self.name(),
                "description": "Returns the current system time in a human-readable format.",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            }
        })
    }

    fn tool_callback(&self) -> bool {
        true
    }

    async fn execute_tool(&self, _args: Value) -> Result<String> {
        let now = chrono::Local::now();
        println!(
            "{}",
            format!(
                "[DEBUG] TimeTool executed\n[Returning] \n{}\n",
                now.to_rfc2822()
            )
            .dimmed()
        );
        Ok(format!("Current system time is: {}", now.to_rfc2822()))
    }
}
