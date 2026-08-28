use std::env;
use std::path::{Path, PathBuf};

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

// The server's bundle directory.
const SERVER_DIST_DIR: &str = "node_modules/@marko/language-server/dist";

// Where to look for the `typescript` npm resolved for the server, holding the
// default `lib.*.d.ts` files. npm nests a dependency only when it cannot dedupe
// it, so a nested copy is the more specific match and wins.
const TYPESCRIPT_LIB_DIRS: [&str; 2] = [
    "node_modules/@marko/language-server/node_modules/typescript/lib",
    "node_modules/typescript/lib",
];

fn work_dir() -> zed::Result<PathBuf> {
    env::current_dir().map_err(|err| err.to_string())
}

// Absolute path to the server script, anchored at the extension work dir.
fn server_script_abs_path() -> zed::Result<PathBuf> {
    Ok(work_dir()?.join(SERVER_SCRIPT_PATH))
}

/// The `typescript/lib` directory npm resolved for the language server.
fn typescript_lib_dir(work_dir: &Path) -> zed::Result<PathBuf> {
    TYPESCRIPT_LIB_DIRS
        .iter()
        .map(|dir| work_dir.join(dir))
        .find(|dir| dir.is_dir())
        .ok_or_else(|| {
            format!(
                "could not find typescript alongside the language server; looked in {}",
                TYPESCRIPT_LIB_DIRS.join(" and ")
            )
        })
}

/// Copy TypeScript's default `lib.*.d.ts` files next to the language server's
/// bundle.
///
/// The server resolves `typescript/package.json` starting from the *project's*
/// tsconfig, and falls back to its own `__dirname` when that fails
/// (`service/script/index.ts`). A project with no `node_modules` therefore gets
/// no default libs at all, producing bogus "Cannot find name 'Date'" errors.
/// The official VS Code extension avoids this by copying these same files into
/// its bundle at build time (`scripts/marko-esbuild.mts`, `copyMarkoBundleAssets`);
/// the npm-published server does not ship them, so we reproduce that step here.
///
/// The libs are taken from the `typescript` npm installed for the server (a
/// transitive dependency it declares as `^6.0.3`), so the lib set always matches
/// the TypeScript the server actually runs. Called only when the server is
/// installed or updated, and always overwrites: the files are small, and this
/// keeps them in step with a changed TypeScript version.
fn copy_default_ts_libs() -> zed::Result<()> {
    let work_dir = work_dir()?;
    let lib_dir = typescript_lib_dir(&work_dir)?;
    let dist_dir = work_dir.join(SERVER_DIST_DIR);

    if !dist_dir.is_dir() {
        return Err(format!(
            "language server bundle not found at {}",
            dist_dir.display()
        ));
    }

    let entries = std::fs::read_dir(&lib_dir).map_err(|err| {
        format!(
            "could not read typescript lib dir {}: {err}",
            lib_dir.display()
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|err| err.to_string())?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };

        if !is_default_lib_file(name) {
            continue;
        }

        std::fs::copy(entry.path(), dist_dir.join(name))
            .map_err(|err| format!("could not copy {name} into the server bundle: {err}"))?;
    }

    Ok(())
}

fn is_default_lib_file(name: &str) -> bool {
    name.starts_with("lib.") && name.ends_with(".d.ts")
}

/// Whether the default libs have already been copied into the server bundle.
///
/// `lib.d.ts` is the entry point TypeScript loads first, so its presence is a
/// good enough proxy for the whole set without stat-ing every file.
fn default_libs_present() -> zed::Result<bool> {
    Ok(work_dir()?.join(SERVER_DIST_DIR).join("lib.d.ts").is_file())
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
                // The install replaces `dist/`, so the libs must be copied in
                // again against the TypeScript this version resolved to.
                copy_default_ts_libs()?;
            }
            // Up to date, so `dist/` is intact and the libs are normally already
            // there. Re-copy only if they are missing, which self-heals an
            // install that predates this step.
            Ok(_) if !default_libs_present()? => copy_default_ts_libs()?,
            Ok(_) => {}
            Err(_) if std::fs::metadata(&script_path).is_ok() => {
                // Registry unreachable but an existing install is present — use it
                // without caching, so the next call retries the version check.
                // Nothing was reinstalled, so the libs from the last successful
                // install are still in place.
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
