#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use review_sweeper_application::AppMetadata;
use thiserror::Error;

#[derive(Debug, Error)]
enum StartupError {
    #[error("loggingの初期化に失敗しました: {0}")]
    Logging(String),
    #[error(transparent)]
    Ui(#[from] review_sweeper_ui_gpui::LaunchError),
}

fn run() -> Result<(), StartupError> {
    tracing_subscriber::fmt::try_init()
        .map_err(|error| StartupError::Logging(error.to_string()))?;

    let metadata = AppMetadata::default();
    tracing::info!(app = metadata.name(), "アプリケーションを起動します");
    review_sweeper_ui_gpui::launch(metadata)?;
    tracing::info!("アプリケーションを終了しました");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Review Sweeperを起動できませんでした: {error}");
        std::process::exit(1);
    }
}
