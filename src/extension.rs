use std::fs;

use zed_extension_api::CodeLabel;
use zed_extension_api::CodeLabelSpan;
use zed_extension_api::DownloadedFileType;
use zed_extension_api::Extension;
use zed_extension_api::GithubReleaseOptions;
use zed_extension_api::LanguageServerId;
use zed_extension_api::LanguageServerInstallationStatus;
use zed_extension_api::Os;
use zed_extension_api::Worktree;
use zed_extension_api::current_platform;
use zed_extension_api::download_file;
use zed_extension_api::latest_github_release;
use zed_extension_api::make_file_executable;
use zed_extension_api::set_language_server_installation_status;

use zed_extension_api::lsp::Completion;
use zed_extension_api::lsp::CompletionKind;

use crate::config::GroovyEnhancedConfig;
use crate::zed;

pub struct GroovyExtension {
    pub cached_binary_path: Option<String>,
}

impl GroovyExtension {
    pub fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
    ) -> zed::Result<String> {
        if let Some(path) = &self.cached_binary_path
            && fs::metadata(path).is_ok_and(|stat| stat.is_file())
        {
            return Ok(path.clone());
        }

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = latest_github_release(
            "valentinegb/groovy-language-server",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let (platform, _arch) = current_platform();
        let asset_name = format!(
            "groovy-language-server-{os}",
            os = match platform {
                Os::Mac => "macOS",
                Os::Linux => "Linux",
                Os::Windows => "Windows",
            },
        );
        let asset_file = format!("{asset_name}.zip");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_file)
            .ok_or_else(|| format!("no asset found matching {asset_file:?}"))?;
        let version_dir = format!("groovy-language-server-{}", release.version);
        let binary_path = format!("{version_dir}/{asset_name}/groovy_language_server_wrapper");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );
            download_file(&asset.download_url, &version_dir, DownloadedFileType::Zip)
                .map_err(|e| format!("failed to download file: {e}"))?;
            make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;

            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;

                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());

        Ok(binary_path)
    }
}

impl Extension for GroovyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id)?;
        let config = GroovyEnhancedConfig::from_lsp_settings(language_server_id, worktree);

        let mut env: Vec<(String, String)> = Vec::new();

        if let Some(ref home_path) = config.java_home {
            env.push(("JAVA_HOME".to_string(), home_path.clone()));
            env.push(("PATH".to_string(), format!("{home_path}/bin:/usr/bin:/bin")));
        }

        Ok(zed::Command {
            command: binary_path,
            args: Vec::new(),
            env,
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Class | CompletionKind::Enum | CompletionKind::Interface => {
                Some(CodeLabel {
                    code: format!("{} variable", completion.label),
                    spans: vec![
                        CodeLabelSpan::code_range(0..completion.label.len()),
                        CodeLabelSpan::literal(format!(" (import {})", completion.detail?), None),
                    ],
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Method => {
                let code = format!("{}()", completion.label);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(0..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            CompletionKind::Variable => {
                let def = "def ";
                let code = format!("{def}{}", completion.label);

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(def.len()..code.len())],
                    code,
                    filter_range: (0..completion.label.len()).into(),
                })
            }
            _ => None,
        }
    }
}
