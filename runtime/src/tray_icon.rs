//! Add a tray icon

/// A tray icon action to be performed by some [`Task`].
#[derive(Debug)]
pub enum Action {
    /// A tray icon event.
    TrayIconEvent(tray_icon::TrayIconEvent),

    /// A tray menu event.
    TrayMenuEvent(tray_icon::menu::MenuEvent),
}


