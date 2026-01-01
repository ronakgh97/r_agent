use anyhow::Result;
use forge::api::dtos::Message;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub name: String,
    pub last_model_used: String,
    pub path: PathBuf,
    pub messages: Vec<Message>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MappedMessage {
    User(String),
    Agent(String),
}

impl Session {
    pub fn new(name: &str, model_used: &str, path: PathBuf) -> Self {
        Session {
            name: name.to_string(),
            last_model_used: model_used.to_string(),
            path: get_default_session_path().unwrap_or(path),
            messages: Vec::new(),
        }
    }

    pub async fn save_to_disk(&self) -> Result<()> {
        let session_data = serde_json::to_string_pretty(self)?;
        let file_name = format!("{}.json", self.name);
        let path = self.path.join(file_name);
        fs::write(path, session_data).await?;
        Ok(())
    }
}

pub async fn load_session(session_name: &str) -> Result<Session> {
    let session_path = get_default_session_path()?;
    let file_name = format!("{}.json", session_name);
    let full_path = session_path.join(file_name);
    let session_data = tokio::fs::read_to_string(&full_path).await?;
    let session: Session = serde_json::from_str(&session_data)?;
    Ok(session)
}

pub fn get_default_session_path() -> Result<PathBuf> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let session_path = home_dir.join(".config").join("r_agent").join("sessions");
    Ok(session_path)
}

pub async fn create_session_dir() -> Result<PathBuf> {
    let session_path = get_default_session_path()?;
    fs::create_dir_all(&session_path).await?;

    Ok(session_path)
}
