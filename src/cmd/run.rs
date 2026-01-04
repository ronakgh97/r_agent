use crate::core::config::load_config;
use crate::core::runner::RunnerContext;
use crate::core::session::Session;
use crate::core::session::{get_default_session_path, load_session};
use anyhow::{Context, Result};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use colored::Colorize;

pub async fn run_agent(
    task: &str,
    _plan: &Option<String>,
    image: &Option<String>,
    config: &str,
    session: &Option<String>,
    context: &Option<String>,
) -> Result<()> {
    println!("\nRunning agent...\n");
    println!("Task: {}", task.to_string().yellow());
    println!("Config: {}", config.to_string().yellow());

    if let Some(image_path) = image {
        let encoded_image = encode_image(image_path)?;
        println!(
            "Image: {} (encoded to {} chars)",
            image_path.to_string().yellow(),
            encoded_image.len().to_string().cyan().bold()
        );
    } else {
        println!("Image: None");
    }

    if let Some(s) = session {
        println!("Session: {}", s);
    } else {
        println!("Session: None");
    }
    if let Some(ctx) = context {
        println!("Context: {} chars", ctx.len().to_string().cyan().bold());
    } else {
        println!("Context: None");
    }

    println!();

    // Load agent config
    let config_body = load_config(config.to_string()).await?;

    let mut session_data = if let Some(session_name) = session {
        let session_path = get_default_session_path()
            .with_context(|| anyhow::anyhow!("Failed to get default session path"))?;
        let full_path = session_path.join(format!("{}.json", session_name));

        // Try to load existing session, or create a new one if it doesn't exist
        let session = if full_path.exists() {
            println!(
                "Loading session: {}\n",
                session_name.to_string().green().bold()
            );
            load_session(session_name)
                .await
                .with_context(|| anyhow::anyhow!("Failed to load session"))?
        } else {
            println!(
                "Creating session: {}\n",
                session_name.to_string().green().bold()
            );
            Session::new(session_name, config, session_path)
        };
        Some(session)
    } else {
        None
    };

    let context = context.clone();

    // Encoded the image
    let image = if let Some(image_path) = image {
        Some(encode_image(image_path)?)
    } else {
        None
    };

    let mut runner_context = RunnerContext::pre_load(&config_body, &session_data, &context, &image)
        .await
        .with_context(|| anyhow::anyhow!("Failed to preload runner context"))?;

    if let Some(ref mut session) = session_data {
        runner_context
            .run_session(task.to_string(), session)
            .await?;
        runner_context.session = Some(session.clone()); // keep context in sync if needed
    } else {
        runner_context.run(task.to_string()).await?;
        // No session to save
    }

    Ok(())
}

pub async fn read_stdin() -> Option<String> {
    use tokio::io::{self, AsyncReadExt};

    if atty::is(atty::Stream::Stdin) {
        return None;
    }

    let mut stdin = io::stdin();
    let mut buffer = String::new();
    stdin
        .read_to_string(&mut buffer)
        .await
        .with_context(|| anyhow::anyhow!("Failed to read from pipe"))
        .ok()?;
    if buffer.trim().is_empty() {
        None
    } else {
        Some(buffer)
    }
}

fn encode_image(image_path: &String) -> Result<String> {
    let image_data = std::fs::read(image_path)
        .with_context(|| anyhow::anyhow!("Failed to read image file: {}", image_path))?;
    let encoded = BASE64_STANDARD.encode(&image_data);
    Ok(encoded)
}
