use std::env;
use std::path::PathBuf;

use zed::LanguageServerId;
use zed_extension_api as zed;

const PACKAGE_NAME: &str = "@marko/language-server";

struct MarkoExtension {
    cached_binary_path: Option<String>,
}

// Relative to the extension's own working directory, where Zed installs npm
// packages. It must be resolved against `env::current_dir()` before being handed
// to Zed: the language server process is spawned with the user's worktree as its
// working directory, so a relative path would resolve against the wrong root.
const SERVER_SCRIPT_PATH: &str = "node_modules/@marko/language-server/bin.js";

// Absolute path to the server script, anchored at the extension work dir.
fn server_script_abs_path() -> zed::Result<PathBuf> {
    Ok(env::current_dir()
        .map_err(|err| err.to_string())?
        .join(SERVER_SCRIPT_PATH))
}

impl MarkoExtension {
    fn server_script_path(&mut self, id: &LanguageServerId) -> zed::Result<String> {
        let script_path = server_script_abs_path()?;
        let script_path = script_path.to_string_lossy().to_string();

        let version_check = zed::npm_package_installed_version(PACKAGE_NAME)
            .and_then(|installed| Ok((installed, zed::npm_package_latest_version(PACKAGE_NAME)?)));

        match version_check {
            Ok((installed_version, latest_version))
                if installed_version.as_deref() != Some(latest_version.as_str()) =>
            {
                zed::set_language_server_installation_status(
                    id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );
                zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
            }
            Ok(_) => {}
            Err(_) if std::fs::metadata(&script_path).is_ok() => {
                // Registry unreachable but an existing install is present — use it
                // without caching, so the next call retries the version check.
                return Ok(script_path);
            }
            Err(err) => return Err(err),
        }

        self.cached_binary_path = Some(script_path.clone());
        Ok(script_path)
    }
}

impl zed::Extension for MarkoExtension {
    fn new() -> Self {
        MarkoExtension {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let script_path = match &self.cached_binary_path {
            Some(path) => path.clone(),
            None => self.server_script_path(language_server_id)?,
        };

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![script_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(MarkoExtension);
