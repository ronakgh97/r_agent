use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "r-agent",
    about = "R-Agent - A pure terminal-based tool for testing my ai-agent library.",
    long_about = "None"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize dotfiles and pre-configs
    Init {
        /// An attempt to fix/valid dotfiles if they are corrupted/bad serialized
        #[arg(long)]
        fix: bool,
    },

    /// Run the AI agent on a task
    Run {
        /// The task prompt (primary input)
        task: String,

        /// The agent's high-level plan/goal
        #[arg(short, long)]
        plan: Option<String>,

        ///Agent Config to use for the agent
        #[arg(short, long)]
        config: String,

        /// Session name to use for persistent memory/session
        #[arg(short, long)]
        session: Option<String>,
    },
}
