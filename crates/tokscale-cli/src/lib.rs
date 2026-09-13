//! Library target for the tokscale CLI.
//!
//! FORK NOTE: upstream `tokscale-cli` declares only a `[[bin]]`, so none of the
//! auth, sync or command logic is reachable from another crate. This target
//! exists so the tokscale-gui Tauri backend can call it in-process rather than
//! shipping the binary as a sidecar. See ADR 0002 in the tokscale-gui repo.
//!
//! Keep this file mechanical — it should stay a thin re-export list so upstream
//! merges conflict as little as possible.

mod shared;
pub use shared::{
    build_date_filter_for_date, normalize_year_filter, saturating_token_total, ClientFilter,
    ClientFlags, DateRangeFlags,
};

pub mod antigravity;
pub mod auth;
pub mod claude_diagnostics;
pub mod commands;
pub mod cursor;
pub mod device;
pub mod hindsight;
pub mod paths;
pub mod process_liveness;
pub mod spawn;
pub mod trae;
pub mod tui;
pub mod warp;
