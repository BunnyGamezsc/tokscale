//! FORK NOTE: not in upstream. Upstream spawns vendor CLIs by bare name, which a
//! macOS app launched from Finder cannot resolve: it inherits launchd's PATH,
//! `/usr/bin:/bin:/usr/sbin:/sbin`. An embedder installs a resolver once. With
//! none installed this is `Command::new(name)`, so the CLI behaves as upstream.
//! See ADR 0006 and 0007 in the tokscale-gui repo.

use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

static RESOLVER: OnceLock<fn(&str) -> PathBuf> = OnceLock::new();

/// Installs the resolver. Only the first call takes effect.
pub fn set_resolver(resolve: fn(&str) -> PathBuf) {
    let _ = RESOLVER.set(resolve);
}

/// `Command::new` for a vendor CLI, through the resolver when one is installed.
pub fn command(name: &str) -> Command {
    match RESOLVER.get() {
        Some(resolve) => Command::new(resolve(name)),
        None => Command::new(name),
    }
}
