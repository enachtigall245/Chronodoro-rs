use anyhow::Result;
use std::{fs, path::PathBuf};
use crate::core::config::Config;

pub fn config_path() -> Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().unwrap_or_else(|| std::path::Path::new("."));
    Ok(dir.join("pomodoro_config.json"))
}

pub fn load_config(path: &PathBuf) -> Result<Config> {
    if path.exists() {
        Ok(serde_json::from_slice(&fs::read(path)?)?)
    } else {
        Ok(Config::default())
    }
}

pub fn save_config(path: &PathBuf, cfg: &Config) -> Result<()> {
    fs::write(path, serde_json::to_vec_pretty(cfg)?)?;
    Ok(())
}