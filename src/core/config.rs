use crate::core::tools::get_default_toolset;
use anyhow::{Context, Result};
use forge::api::agents::{Agent, AgentBuilder};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

pub const SYSTEM_PROMPT: &str = r#"
You are an expert AI coding agent operating inside a local software project.

You have access to a set of tools that allow you to:
- List files and directories in the current project
- Tree-visual the project structure
- Search code using ripgrep-style search
- Read files from disk
- Inspect git diffs, logs and repository state
- Determine the current working directory
- Check background process status

IMPORTANT TOOL GUIDELINES:
- Use tools whenever information is needed from the project instead of asking the user
- Assume the current working directory is the project root unless proven otherwise
- Always prefer reading files over guessing their contents
- Do NOT assume file contents without reading them
- When no context provided, Use your tools and go through the codebase methodically, read file contents or check all directories, especially README and docs to gather information
- You cannot directly edit files for now
- Treat all tools as safe, read-only operations unless stated otherwise

CRITICAL BEHAVIOR RULE:
- Never ask the user what to inspect.
- Never ask permission to use tools.
- Never ignore images when provided, if you are vision capable, always analyze them as part of your inspection.
- If a task requires project understanding, you MUST immediately use tools.
- Do not explain available tools to the user.
- Do not ask follow-up questions until after inspection is complete.

Your responsibilities:
- Understand and explain existing codebases
- Debug issues by inspecting real project files
- Review code for correctness, performance, and security
- Suggest concrete improvements grounded in the actual code
- Explain technical concepts clearly and accurately

When responding:
- Be concise but thorough
- Follow existing code style and project conventions
- Explain reasoning when making architectural or design suggestions
- Avoid hallucination; verify by reading files when unsure
- If information is missing and tools cannot provide it, ask the user

When writing or suggesting code:
- Use idiomatic patterns for the language
- Prefer clarity over cleverness
- Handle errors explicitly
- Consider performance and maintainability
"#;

pub fn default_agents() -> Vec<Agent> {
    vec![
        AgentBuilder::new()
            .model("qwen/qwen3-8b")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("qwen/qwen3-vl-8b")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("zai-org/glm-4.6v-flash")
            .url("http://localhost:1234/v1")
            .api_key("local")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
        AgentBuilder::new()
            .model("qwen/qwen3-coder:free")
            .url("https://openrouter.ai/v1")
            .api_key("YOUR_OPENROUTER_API_KEY")
            .system_prompt(SYSTEM_PROMPT)
            .tool_registry(Arc::new(get_default_toolset()))
            .build()
            .unwrap(),
    ]
}

pub async fn get_agent_configs(config_dir: PathBuf, model: &str) -> Result<Agent> {
    let sanitized_name = model.replace("/", "_").replace(":", "_");
    let file_format = format!("{}.toml", sanitized_name);

    // Get the config in dir
    let file_path = config_dir.join(file_format);
    let config_data = tokio::fs::read_to_string(&file_path).await?;
    let agent_builder = AgentBuilder::load_from_toml(&config_data)?;
    let agent = agent_builder.build()?;
    Ok(agent)
}

pub async fn save_default_agent_configs(agent: &Agent, path: PathBuf) -> Result<()> {
    let agent_str = AgentBuilder::convert_to_builder(agent).to_toml_string()?;

    // Save the toml - sanitize filename by replacing invalid characters
    let sanitized_name = agent.model.replace("/", "_").replace(":", "_");
    let file_name = format!("{}.toml", sanitized_name);
    let file_path = path.join(file_name);
    fs::write(file_path, agent_str).await?;

    Ok(())
}

pub async fn load_config(agent_config: String) -> Result<String> {
    let config_dir = get_default_config_path()
        .with_context(|| anyhow::anyhow!("Failed to get default config path"))?;
    let config_file_name = format!("{}.toml", agent_config);
    let config_path = config_dir.join(config_file_name);
    let config_body = tokio::fs::read_to_string(&config_path)
        .await
        .with_context(|| anyhow::anyhow!("Failed to read config file"))?;
    Ok(config_body)
}

pub fn get_default_config_path() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let config_path = home_dir.join(".config").join("r_agent").join("config");
    Ok(config_path)
}

pub async fn create_config_dir() -> Result<PathBuf> {
    let config_path = get_default_config_path()?;
    fs::create_dir_all(&config_path).await?;

    Ok(config_path)
}
