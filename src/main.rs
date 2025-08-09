use anyhow::Result;
use chronodoro::{app::PomodoroApp, persistence::config_path};

fn main() {
    let cfg_path = config_path().unwrap_or_else(|e| {
        eprintln!("Config path error: {e}. Using local JSON.");
        std::path::PathBuf::from("pomodoro_config.json")
    });

    let native_options = eframe::NativeOptions::default();

    // Explicit type = exactly what run_native expects
    let app_creator: Box<
        dyn FnOnce(&eframe::CreationContext<'_>)
            -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>>
    > = Box::new({
        let cfg_path = cfg_path.clone();
        move |_cc| Ok(Box::new(PomodoroApp::new(cfg_path)) as Box<dyn eframe::App>)
    });

    if let Err(e) = eframe::run_native("Chronodoro", native_options, app_creator) {
        eprintln!("Failed to start app: {e}");
    }
}

