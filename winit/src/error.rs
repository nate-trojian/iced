use crate::futures::futures;
use crate::graphics;

/// An error that occurred while running an application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The futures executor could not be created.
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(futures::io::Error),

    /// The application window could not be created.
    #[error("the application window could not be created")]
    WindowCreationFailed(winit::error::OsError),

    /// The application graphics context could not be created.
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(graphics::Error),

    /// The application tray icon could not be created
    #[cfg(feature = "tray-icon")]
    #[error("the application tray icon could not be created")]
    TrayIconCreationFailed(#[from] iced_runtime::tray_icon::Error),
}

impl From<graphics::Error> for Error {
    fn from(error: iced_graphics::Error) -> Error {
        Error::GraphicsCreationFailed(error)
    }
}
