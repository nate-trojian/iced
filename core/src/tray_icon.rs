//! Tray icon events

use crate::{mouse::Button, Point, Rectangle};

/// A tray icon interaction
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Tray icon click started
    MouseButtonPressed {
        /// Id of the tray icon which triggered this event.
        id: String,
        /// Position of the mouse cursor
        position: Point,
        /// Bounding rectangle of the tray icon
        rect: Rectangle,
        /// Mouse button which triggered this event [Left, Middle, Right]
        button: Button,
    },
    /// Tray icon click ended
    MouseButtonReleased {
        /// Id of the tray icon which triggered this event.
        id: String,
        /// Position of the mouse cursor
        position: Point,
        /// Bounding rectangle of the tray icon
        rect: Rectangle,
        /// Mouse button which triggered this event [Left, Middle, Right]
        button: Button,
    },
    /// Tray icon double clicked
    DoubleClicked {
        /// Id of the tray icon which triggered this event.
        id: String,
        /// Position of the mouse cursor
        position: Point,
        /// Bounding rectangle of the tray icon
        rect: Rectangle,
        /// Mouse button which triggered this event [Left, Middle, Right]
        button: Button,
    },
    /// Mouse entered tray icon
    MouseEntered {
        /// Id of the tray icon which triggered this event.
        id: String,
        /// Position of the mouse cursor
        position: Point,
        /// Bounding rectangle of the tray icon
        rect: Rectangle,
    },
    /// Mouse moved over tray icon
    MouseMoved {
        /// Id of the tray icon which triggered this event.
        id: String,
        /// Position of the mouse cursor
        position: Point,
        /// Bounding rectangle of the tray icon
        rect: Rectangle,
    },
    /// Mouse exited tray icon
    MouseExited {
        /// Id of the tray icon which triggered this event.
        id: String,
        /// Position of the mouse cursor
        position: Point,
        /// Bounding rectangle of the tray icon
        rect: Rectangle,
    },
    /// Tray icon menu item clicked
    MenuItemClicked {
        /// Id of the tray icon which triggered this event.
        id: String
    }
}