use std::fmt;

use serde::{Deserialize, Serialize};

/// Enum representing available Discord API versions
///
/// [DEVELOPER PORTAL #API Versioning](https://discord.com/developers/docs/reference#api-versioning)
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum DiscordApiVersion {
    /// Version 6 - Deprecated (DEFAULT)
    V6 = 6,
    /// Version 7 - Deprecated
    V7 = 7,
    /// Version 8 - Deprecated
    V8 = 8,
    /// Version 9 - Deprecated
    V9 = 9,
    /// Version 10 - Available
    V10 = 10,
}

impl fmt::Display for DiscordApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as i16)
    }
}

impl DiscordApiVersion {
    /// Returns a vector of available Discord API versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use membrs_lib::api::DiscordApiVersion;
    ///
    /// let available_versions = DiscordApiVersion::available_versions();
    /// println!("Available Versions: {:?}", available_versions);
    /// ```
    pub fn available_versions() -> Vec<DiscordApiVersion> {
        vec![
            DiscordApiVersion::V6,
            DiscordApiVersion::V7,
            DiscordApiVersion::V8,
            DiscordApiVersion::V9,
            DiscordApiVersion::V10,
        ]
    }

    /// Selects the Discord API version based on the provided version string.
    /// Returns `Some(DiscordApiVersion)` if the version is available, otherwise `None`.
    ///
    /// # Arguments
    ///
    /// * `version` - A string slice representing the version number.
    ///
    /// # Examples
    ///
    /// ```
    /// use membrs_lib::api::DiscordApiVersion;
    ///
    /// let selected_version = DiscordApiVersion::select_version("10");
    /// println!("Selected Version: {:?}", selected_version);
    /// ```
    pub fn select_version(version: &str) -> Option<DiscordApiVersion> {
        match version.trim().to_lowercase().as_str() {
            "6" => Some(DiscordApiVersion::V6),
            "7" => Some(DiscordApiVersion::V7),
            "8" => Some(DiscordApiVersion::V8),
            "9" => Some(DiscordApiVersion::V9),
            "10" => Some(DiscordApiVersion::V10),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscordAPi {
    url: String,
    version: DiscordApiVersion,
}

impl Default for DiscordAPi {
    fn default() -> Self {
        Self {
            url: "https://discord.com/api".to_string(),
            version: DiscordApiVersion::V6,
        }
    }
}

impl DiscordAPi {
    pub fn new(url: &str, version: DiscordApiVersion) -> Self {
        Self {
            url: url.to_string(),
            version,
        }
    }

    pub fn build_url(&self) -> String {
        format!("{}/v{}", self.url, self.version)
    }

    pub fn append_path(&self, path: &str) -> String {
        format!("{}/{}", self.build_url(), path.trim_start_matches('/'))
    }
}

#[macro_export]
macro_rules! append_path {
    ($api:expr, $($part:expr),*) => {{
        let path = format!($($part),*);
        &$api.append_path(&path)
    }};
}
