use anyhow::Result;
use clap::Parser;
use r_agent::args::{Args, Commands};
use r_agent::cmd::ascii::run_ascii_art;
use r_agent::cmd::init::run_init;
use r_agent::cmd::run::{read_stdin, run_agent};

#[tokio::main]
pub async fn main() -> Result<()> {
    let piped_input = read_stdin().await;

    let cli_args = Args::parse();

    match cli_args.command {
        Some(Commands::Init { fix }) => {
            run_init(fix).await?;
        }
        Some(Commands::Run {
            task,
            plan,
            config,
            session,
        }) => {
            let task_str = task.unwrap_or_else(|| {
                eprintln!("Error: Task is required");
                eprintln!("Usage: ragent run <TASK> --config <CONFIG>");
                eprintln!(
                    "Example: cat src/cmd/run.rs | ragent run \"explain this\" --config qwen_qwen3-8b\n"
                );
                std::process::exit(1);
            });
            run_agent(&task_str, &plan, &config, &session, &piped_input).await?;
        }

        _ => {
            run_ascii_art().await;
        }
    }

    Ok(())
}
