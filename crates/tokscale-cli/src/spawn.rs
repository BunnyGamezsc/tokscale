//! FORK NOTE: not in upstream. Upstream spawns vendor CLIs by bare name, which a
//! macOS app launched from Finder cannot resolve: it inherits launchd's PATH,
//! `/usr/bin:/bin:/usr/sbin:/sbin`. An embedder installs a resolver once. With
//! none installed this is `Command::new(name)`, so the CLI behaves as upstream.
//! See ADR 0006 and 0007 in the tokscale-gui repo.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

static RESOLVER: OnceLock<fn(&str) -> PathBuf> = OnceLock::new();

/// Installs the resolver. Only the first call takes effect.
pub fn set_resolver(resolve: fn(&str) -> PathBuf) {
    let _ = RESOLVER.set(resolve);
}

/// `Command::new` for a vendor CLI, through the resolver when one is installed.
///
/// The child also gets the binary's own directory at the front of its PATH: an
/// nvm-installed CLI is a `#!/usr/bin/env node` script, and launchd's PATH has
/// no `node`, but the directory the script lives in does.
pub fn command(name: &str) -> Command {
    let Some(resolve) = RESOLVER.get() else {
        return Command::new(name);
    };
    let program = resolve(name);
    let mut command = Command::new(&program);
    if let Some(path) = path_with_dir_of(&program, std::env::var_os("PATH")) {
        command.env("PATH", path);
    }
    command
}

/// `inherited` with `program`'s directory in front, or `None` for a bare name.
fn path_with_dir_of(program: &Path, inherited: Option<OsString>) -> Option<OsString> {
    let dir = program.parent().filter(|dir| !dir.as_os_str().is_empty())?;
    let rest = inherited.iter().flat_map(std::env::split_paths);
    std::env::join_paths(std::iter::once(dir.to_path_buf()).chain(rest)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_binary_directory_goes_in_front_of_the_inherited_path() {
        let path = path_with_dir_of(
            Path::new("/Users/x/.nvm/versions/node/v26.7.0/bin/codex"),
            Some("/usr/bin:/bin".into()),
        );
        assert_eq!(path, Some("/Users/x/.nvm/versions/node/v26.7.0/bin:/usr/bin:/bin".into()));
        assert_eq!(path_with_dir_of(Path::new("/opt/homebrew/bin/gh"), None), Some("/opt/homebrew/bin".into()));
        assert_eq!(path_with_dir_of(Path::new("codex"), Some("/usr/bin".into())), None);
    }
}
