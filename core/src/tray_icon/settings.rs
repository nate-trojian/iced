//! Tray icon settings

#[cfg(feature = "tray-icon")]
use crate::tray_icon::Error;

use crate::Size;

/// Tray icon settings
#[derive(Debug)]
pub struct Settings {
    /// Title of the icon
    pub title: Option<String>,
    /// Icon to show (not available on web)
    pub icon: Option<Icon>,
    /// Tooltip to show on hover
    pub tooltip: Option<String>,
}

#[cfg(feature = "tray-icon")]
impl TryFrom<Settings> for tray_icon::TrayIconAttributes {
    type Error = Error;
    fn try_from(value: Settings) -> Result<Self, Self::Error> {
        let mut attrs = Self::default();
        if let Some(title) = value.title {
            attrs.title = Some(title.clone());
        }
        if let Some(icon) = value.icon {
            attrs.icon = Some(icon.try_into()?);
        }
        if let Some(tooltip) = value.tooltip {
            attrs.tooltip = Some(tooltip.clone());
        }
        Ok(attrs)
    }
}

/// Icon data
#[derive(Debug)]
#[allow(dead_code)]
pub struct Icon {
    /// RGBA byte data of icon image
    pub rgba: Vec<u8>,
    /// Size of icon image
    pub size: Size<u32>,
}

#[cfg(feature = "tray-icon")]
impl TryFrom<Icon> for tray_icon::Icon {
    type Error = Error;
    fn try_from(value: Icon) -> Result<Self, Self::Error> {
        Self::from_rgba(value.rgba, value.size.width, value.size.height)
            .map_err(Self::Error::from)
    }
}
