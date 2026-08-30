//! GPUI 固有型を閉じ込める Windows native UI adapter。

use review_sweeper_application::AppMetadata;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LaunchError {
    #[cfg(windows)]
    #[error("GPUIウィンドウを作成できませんでした: {0}")]
    Window(String),
    #[cfg(not(windows))]
    #[error("Review SweeperのGUIは現在Windows native環境だけをサポートします")]
    UnsupportedPlatform,
}

#[cfg(windows)]
mod windows {
    use std::sync::{Arc, Mutex};

    use gpui::{
        App, Application, Bounds, Context, Render, Window, WindowBounds, WindowOptions, div,
        prelude::*, px, rgb, size,
    };

    use super::{AppMetadata, LaunchError};

    struct RootView {
        metadata: AppMetadata,
    }

    impl Render for RootView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(rgb(0x0d1117))
                .text_color(rgb(0xf0f6fc))
                .text_xl()
                .child(self.metadata.name())
        }
    }

    /// GPUI event loop を開始し、最後の window が閉じられるまで block する。
    pub fn launch(metadata: AppMetadata) -> Result<(), LaunchError> {
        let startup_error = Arc::new(Mutex::new(None));
        let reported_error = Arc::clone(&startup_error);

        Application::new().run(move |cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
            let result = cx.open_window(
                WindowOptions {
                    focus: true,
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some(metadata.name().into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_window, cx| cx.new(|_| RootView { metadata }),
            );

            match result {
                Ok(_) => {
                    tracing::info!("GPUIウィンドウを作成しました");
                    cx.activate(true);
                }
                Err(error) => {
                    tracing::error!(%error, "GPUIウィンドウを作成できませんでした");
                    *reported_error.lock().expect("startup error mutex poisoned") =
                        Some(error.to_string());
                    cx.quit();
                }
            }
        });

        match startup_error
            .lock()
            .expect("startup error mutex poisoned")
            .take()
        {
            Some(error) => Err(LaunchError::Window(error)),
            None => Ok(()),
        }
    }
}

#[cfg(windows)]
pub use windows::launch;

#[cfg(not(windows))]
pub fn launch(_metadata: AppMetadata) -> Result<(), LaunchError> {
    Err(LaunchError::UnsupportedPlatform)
}
