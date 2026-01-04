use anyhow::Result;
use clap::Parser;
use colored::Colorize;
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
            image,
            plan,
            config,
            session,
        }) => {
            let task_str = task.unwrap_or_else(|| {
                eprintln!("{}", " Error: Task is required".to_string().red());
                eprintln!(" Usage: ragent run {} --config {} --image {}",  "<TASK>".to_string().yellow() ,"<CONFIG>".to_string().yellow(), "<IMAGE_URL> OR <PATH>".to_string().yellow());
                eprintln!(" Example: cat Cargo.toml | ragent run \"explain the crates used\" --config qwen_qwen3-8b");
                eprintln!("             {}", "↑ ↑ ↑ ↑ ↑ ↑ -> Sends as context from piped input".to_string().green());
                std::process::exit(1);
            });
            run_agent(&task_str, &plan, &image, &config, &session, &piped_input).await?;
        }

        _ => {
            run_ascii_art().await;
        }
    }

    Ok(())
}

// fn borrow_checker() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &mut s;
//     println!("{} {}", r1, r2);
// }
