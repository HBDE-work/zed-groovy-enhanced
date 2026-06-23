use serde::Deserialize;

use zed_extension_api::LanguageServerId;
use zed_extension_api::Worktree;
use zed_extension_api::serde_json;

use zed_extension_api::settings::LspSettings;

/// Mirrors the JSON structure under `lsp.groovy-enhanced.settings` in Zed's settings.json
///
/// ```json
/// {
///   "lsp": {
///     "groovy-enhanced": {
///       "settings": {
///         "javaHome": "/usr/lib/jvm/java-11-openjdk-amd64"
///       }
///     }
///   }
/// }
/// ```
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CfgStructure {
    java_home: Option<String>,
}

/// Parsed, typed configuration for the groovy-enhanced extension
#[derive(Debug, Default)]
pub struct GroovyEnhancedConfig {
    pub java_home: Option<String>,
}

impl GroovyEnhancedConfig {
    /// Parse the LSP settings JSON into a typed config
    ///
    /// Returns a default (all-None) config if settings are missing or malformed
    pub fn from_lsp_settings(language_server_id: &LanguageServerId, worktree: &Worktree) -> Self {
        let cfg = Self::parse_settings(language_server_id, worktree).unwrap_or_default();

        Self {
            java_home: cfg.java_home,
        }
    }

    fn parse_settings(
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Option<CfgStructure> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree).ok()?;
        let settings_value = lsp_settings.settings?;
        serde_json::from_value(settings_value).ok()
    }
}
