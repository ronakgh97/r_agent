use crate::core::config::get_default_config_path;
use anyhow::Result;
use colored::Colorize;

pub async fn run_ascii_art() {
    let ascii_art = r#"          
                                     ██   
    ████▄    ▀▀█▄ ▄████ ▄█▀█▄ ████▄ ▀██▀▀ 
    ██ ▀▀   ▄█▀██ ██ ██ ██▄█▀ ██ ██  ██   
    ██      ▀█▄██ ▀████ ▀█▄▄▄ ██ ██  ██   
                     ██                   
                   ▀▀▀                           
    "#;

    println!("{}\n", ascii_art.to_string().magenta());
    let total_configs = get_total_configs().unwrap_or(0);
    println!(" Total Configs: {}\n", total_configs.to_string().cyan());
    let total_sessions = get_total_sessions().unwrap_or(0);
    println!(" Total Sessions: {}\n", total_sessions.to_string().cyan());
    println!(
        " Github: {}\n",
        "https://github.com/ronakgh97/r-agent".to_string().cyan()
    );
}

fn get_total_configs() -> Result<usize> {
    let configs_dir = get_default_config_path()?;
    let entries = std::fs::read_dir(configs_dir)?;
    let count = entries.count();
    Ok(count)
}

fn get_total_sessions() -> Result<usize> {
    let sessions_dir = crate::core::session::get_default_session_path()?;
    let entries = std::fs::read_dir(sessions_dir)?;
    let count = entries.count();
    Ok(count)
}
