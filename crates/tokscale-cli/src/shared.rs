//! Items shared between the `tokscale` binary and the `tokscale_cli` library.
//!
//! FORK NOTE: these were defined inline in `main.rs` upstream. They live here so
//! sibling modules can keep reaching them as `crate::ClientFilter` /
//! `crate::saturating_token_total` from both the binary and the library target.

use clap::{Args, ValueEnum};
use tokscale_core::ClientId;

/// Client identifiers exposed via `--client`.
///
/// Mirrors `tokscale_core::ClientId` plus the `Synthetic` meta-client. We
/// duplicate the variant set on the CLI side so `tokscale-core` stays free of
/// CLI-parsing dependencies and so `Synthetic` (which has no scan path of its
/// own) can be treated as a first-class filter value without changing core
/// invariants.
///
/// Variant order intentionally mirrors `ClientId::ALL` declaration order so
/// the TUI source picker, `--help`'s `[possible values: ...]` listing, and
/// any future iteration over `ClientFilter::value_variants()` agree on a
/// single chronological ordering. `Synthetic` is appended at the end since
/// it has no `ClientId` counterpart.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[value(rename_all = "lowercase")]
pub enum ClientFilter {
    Opencode,
    Claude,
    Codex,
    Cursor,
    Gemini,
    Amp,
    Droid,
    Openclaw,
    Pi,
    Kimi,
    Qwen,
    Roocode,
    Kilocode,
    Mux,
    Kilo,
    Crush,
    Hermes,
    Copilot,
    Goose,
    Codebuff,
    Antigravity,
    Zed,
    Kiro,
    #[value(name = "trae")]
    Trae,
    Warp,
    Cline,
    #[value(name = "9router")]
    NineRouter,
    Gjc,
    Grok,
    Jcode,
    Commandcode,
    Micode,
    #[value(name = "antigravity-cli")]
    AntigravityCli,
    Junie,
    Zcode,
    Opencodereview,
    Codebuddy,
    Workbuddy,
    #[value(name = "devin-cli")]
    DevinCli,
    #[value(name = "devin-desktop")]
    DevinDesktop,
    Senpi,
    #[value(alias = "auggie")]
    Augment,
    Kimchi,
    Reasonix,
    #[value(name = "prime-agent")]
    PrimeAgent,
    Freebuff,
    CherryStudio,
    Dsh,
    Mcode,
    Fx,
    Omp,
    LmStudio,
    Unsloth,
    Hindsight,
    Synthetic,
}

impl ClientFilter {
    /// Returns the canonical lowercase identifier consumed by
    /// `tokscale_core` filter lists. Must match `ClientId::as_str` for every
    /// variant that has a corresponding `ClientId`.
    pub fn as_filter_str(&self) -> &'static str {
        match self {
            Self::Opencode => "opencode",
            Self::Claude => "claude",
            Self::Codex => "codex",
            Self::Cursor => "cursor",
            Self::Gemini => "gemini",
            Self::Amp => "amp",
            Self::Droid => "droid",
            Self::Openclaw => "openclaw",
            Self::Pi => "pi",
            Self::Kimi => "kimi",
            Self::Qwen => "qwen",
            Self::Roocode => "roocode",
            Self::Kilocode => "kilocode",
            Self::Mux => "mux",
            Self::Kilo => "kilo",
            Self::Crush => "crush",
            Self::Hermes => "hermes",
            Self::Copilot => "copilot",
            Self::Goose => "goose",
            Self::Codebuff => "codebuff",
            Self::Antigravity => "antigravity",
            Self::Zed => "zed",
            Self::Kiro => "kiro",
            Self::Trae => "trae",
            Self::Warp => "warp",
            Self::Cline => "cline",
            Self::Gjc => "gjc",
            Self::NineRouter => "9router",
            Self::Grok => "grok",
            Self::Jcode => "jcode",
            Self::Commandcode => "commandcode",
            Self::Micode => "micode",
            Self::AntigravityCli => "antigravity-cli",
            Self::Junie => "junie",
            Self::Zcode => "zcode",
            Self::Opencodereview => "opencodereview",
            Self::Codebuddy => "codebuddy",
            Self::Workbuddy => "workbuddy",
            Self::DevinCli => "devin-cli",
            Self::DevinDesktop => "devin-desktop",
            Self::Senpi => "senpi",
            Self::Augment => "augment",
            Self::Kimchi => "kimchi",
            Self::Reasonix => "reasonix",
            Self::PrimeAgent => "prime-agent",
            Self::Freebuff => "freebuff",
            Self::CherryStudio => "cherrystudio",
            Self::Dsh => "dsh",
            Self::Mcode => "mcode",
            Self::Fx => "fx",
            Self::Omp => "omp",
            Self::LmStudio => "lmstudio",
            Self::Unsloth => "unsloth",
            Self::Hindsight => "hindsight",
            Self::Synthetic => "synthetic",
        }
    }

    /// Convert to the corresponding `ClientId`, or `None` for the
    /// `Synthetic` meta-client which has no scan path of its own.
    ///
    /// Used at boundaries where TUI state (`HashSet<ClientFilter>`) needs
    /// to feed core APIs that still consume `Vec<ClientId>`.
    pub fn to_client_id(self) -> Option<tokscale_core::ClientId> {
                match self {
            Self::Opencode => Some(ClientId::OpenCode),
            Self::Claude => Some(ClientId::Claude),
            Self::Codex => Some(ClientId::Codex),
            Self::Cursor => Some(ClientId::Cursor),
            Self::Gemini => Some(ClientId::Gemini),
            Self::Amp => Some(ClientId::Amp),
            Self::Droid => Some(ClientId::Droid),
            Self::Openclaw => Some(ClientId::OpenClaw),
            Self::Pi => Some(ClientId::Pi),
            Self::Kimi => Some(ClientId::Kimi),
            Self::Qwen => Some(ClientId::Qwen),
            Self::Roocode => Some(ClientId::RooCode),
            Self::Kilocode => Some(ClientId::KiloCode),
            Self::Mux => Some(ClientId::Mux),
            Self::Kilo => Some(ClientId::Kilo),
            Self::Crush => Some(ClientId::Crush),
            Self::Hermes => Some(ClientId::Hermes),
            Self::Copilot => Some(ClientId::Copilot),
            Self::Goose => Some(ClientId::Goose),
            Self::Codebuff => Some(ClientId::Codebuff),
            Self::Antigravity => Some(ClientId::Antigravity),
            Self::Zed => Some(ClientId::Zed),
            Self::Kiro => Some(ClientId::Kiro),
            Self::Trae => Some(ClientId::Trae),
            Self::Warp => Some(ClientId::Warp),
            Self::Cline => Some(ClientId::Cline),
            Self::Gjc => Some(ClientId::Gjc),
            Self::NineRouter => Some(ClientId::Gjc),
            Self::Grok => Some(ClientId::Grok),
            Self::Jcode => Some(ClientId::Jcode),
            Self::Commandcode => Some(ClientId::CommandCode),
            Self::Micode => Some(ClientId::MiMoCode),
            Self::AntigravityCli => Some(ClientId::AntigravityCli),
            Self::Junie => Some(ClientId::Junie),
            Self::Zcode => Some(ClientId::Zcode),
            Self::Opencodereview => Some(ClientId::OpenCodeReview),
            Self::Codebuddy => Some(ClientId::CodeBuddy),
            Self::Workbuddy => Some(ClientId::WorkBuddy),
            Self::DevinCli => Some(ClientId::DevinCli),
            Self::DevinDesktop => Some(ClientId::DevinDesktop),
            Self::Senpi => Some(ClientId::Senpi),
            Self::Augment => Some(ClientId::Augment),
            Self::Kimchi => Some(ClientId::Kimchi),
            Self::Reasonix => Some(ClientId::Reasonix),
            Self::PrimeAgent => Some(ClientId::PrimeAgent),
            Self::Freebuff => Some(ClientId::Freebuff),
            Self::CherryStudio => Some(ClientId::CherryStudio),
            Self::Dsh => Some(ClientId::Dsh),
            Self::Mcode => Some(ClientId::Mcode),
            Self::Fx => Some(ClientId::Fx),
            Self::Omp => Some(ClientId::Omp),
            Self::LmStudio => Some(ClientId::LmStudio),
            Self::Unsloth => Some(ClientId::Unsloth),
            Self::Hindsight => Some(ClientId::Hindsight),
            Self::Synthetic => None,
        }
    }

    /// Lift a `ClientId` back into a `ClientFilter`. Total inverse of
    /// `to_client_id` for non-`Synthetic` variants.
    pub fn from_client_id(client: tokscale_core::ClientId) -> Self {
                match client {
            ClientId::OpenCode => Self::Opencode,
            ClientId::Claude => Self::Claude,
            ClientId::Codex => Self::Codex,
            ClientId::Cursor => Self::Cursor,
            ClientId::Gemini => Self::Gemini,
            ClientId::Amp => Self::Amp,
            ClientId::Droid => Self::Droid,
            ClientId::OpenClaw => Self::Openclaw,
            ClientId::Pi => Self::Pi,
            ClientId::Kimi => Self::Kimi,
            ClientId::Qwen => Self::Qwen,
            ClientId::RooCode => Self::Roocode,
            ClientId::KiloCode => Self::Kilocode,
            ClientId::Mux => Self::Mux,
            ClientId::Kilo => Self::Kilo,
            ClientId::Crush => Self::Crush,
            ClientId::Hermes => Self::Hermes,
            ClientId::Copilot => Self::Copilot,
            ClientId::Goose => Self::Goose,
            ClientId::Codebuff => Self::Codebuff,
            ClientId::Antigravity => Self::Antigravity,
            ClientId::Zed => Self::Zed,
            ClientId::Kiro => Self::Kiro,
            ClientId::Trae => Self::Trae,
            ClientId::Warp => Self::Warp,
            ClientId::Cline => Self::Cline,
            ClientId::Gjc => Self::Gjc,
            ClientId::Grok => Self::Grok,
            ClientId::Jcode => Self::Jcode,
            ClientId::CommandCode => Self::Commandcode,
            ClientId::MiMoCode => Self::Micode,
            ClientId::AntigravityCli => Self::AntigravityCli,
            ClientId::Junie => Self::Junie,
            ClientId::Zcode => Self::Zcode,
            ClientId::OpenCodeReview => Self::Opencodereview,
            ClientId::CodeBuddy => Self::Codebuddy,
            ClientId::WorkBuddy => Self::Workbuddy,
            ClientId::DevinCli => Self::DevinCli,
            ClientId::DevinDesktop => Self::DevinDesktop,
            ClientId::Senpi => Self::Senpi,
            ClientId::Augment => Self::Augment,
            ClientId::Kimchi => Self::Kimchi,
            ClientId::Reasonix => Self::Reasonix,
            ClientId::PrimeAgent => Self::PrimeAgent,
            ClientId::Freebuff => Self::Freebuff,
            ClientId::CherryStudio => Self::CherryStudio,
            ClientId::Dsh => Self::Dsh,
            ClientId::Mcode => Self::Mcode,
            ClientId::Fx => Self::Fx,
            ClientId::Omp => Self::Omp,
            ClientId::LmStudio => Self::LmStudio,
            ClientId::Unsloth => Self::Unsloth,
            ClientId::Hindsight => Self::Hindsight,
        }
    }

    /// Parse a canonical lowercase identifier (the same form
    /// `as_filter_str` returns) into a `ClientFilter`. Returns `None` for
    /// any unknown id so callers can drop unrecognized settings entries
    /// without erroring.
    pub fn from_filter_str(s: &str) -> Option<Self> {
        // Canonical ids match as_filter_str. A few product aliases map onto
        // the same ClientFilter (e.g. "auggie" -> Augment).
        if s == "auggie" {
            return Some(Self::Augment);
        }
        Self::value_variants()
            .iter()
            .copied()
            .find(|f| f.as_filter_str() == s)
    }

    /// The "no filter" default set: every real client, with `Synthetic`
    /// **excluded**. Matches the pre-refactor behavior where a missing
    /// filter scanned every `ClientId` but did NOT post-process synthetic
    /// (synthetic detection has always been opt-in because it
    /// re-attributes messages from other clients to a different bucket).
    ///
    /// Single source of truth: every code path that needs a default
    /// filter (TUI launch, `submit` warm cache, etc.) must consult this
    /// so the cache key, the in-app state, and the loader filter all
    /// agree. Drift between them produces stale-cache misses on every
    /// launch.
    pub fn default_set() -> std::collections::HashSet<Self> {
        Self::value_variants()
            .iter()
            .copied()
            .filter(|f| !matches!(f, Self::Synthetic | Self::NineRouter))
            .collect()
    }
}

/// Saturating sum of the four billable token buckets (input/output/cache
/// read/cache write) used throughout the display layer for per-row and
/// grand-total token counts. tokscale-core saturates these fields at the
/// per-message and per-entry level (see `TokenBreakdown::total` and
/// `model_report_token_totals`), so a corrupt/misbehaving source can
/// legitimately clamp a bucket to `i64::MAX`; combining up to four such
/// buckets with plain `+` can then overflow (debug panic / release wrap).
/// `saturating_add` keeps this fold a no-op for real token counts and only
/// changes behavior in that already-degraded case.
pub fn saturating_token_total(input: i64, output: i64, cache_read: i64, cache_write: i64) -> i64 {
    input
        .saturating_add(output)
        .saturating_add(cache_read)
        .saturating_add(cache_write)
}

#[derive(Args, Clone, Debug, Default)]
pub struct ClientFlags {
    /// Canonical client filter. Repeatable or comma-separated.
    /// Example: `--client opencode,claude` or `-c opencode -c claude`.
    #[arg(
        id = "client_filter",
        long = "client",
        short = 'c',
        value_name = "CLIENTS",
        value_enum,
        value_delimiter = ',',
        action = clap::ArgAction::Append,
        ignore_case = true,
        help = "Filter by client(s). Repeatable or comma-separated (e.g. -c opencode,claude)."
    )]
    pub clients: Vec<ClientFilter>,
}

#[derive(Args, Clone, Debug, Default)]
pub struct DateRangeFlags {
    #[arg(
        long,
        help = "Show only today's usage",
        conflicts_with_all = ["yesterday", "week", "month", "since", "until", "year"]
    )]
    pub today: bool,
    #[arg(
        long,
        help = "Show only yesterday's usage",
        conflicts_with_all = ["week", "month", "since", "until", "year"]
    )]
    pub yesterday: bool,
    #[arg(
        long,
        help = "Show last 7 days",
        conflicts_with_all = ["month", "since", "until", "year"]
    )]
    pub week: bool,
    #[arg(
        long,
        help = "Show current month",
        conflicts_with_all = ["since", "until", "year"]
    )]
    pub month: bool,
    #[arg(long, help = "Start date (YYYY-MM-DD)")]
    pub since: Option<String>,
    #[arg(long, help = "End date (YYYY-MM-DD)")]
    pub until: Option<String>,
    #[arg(long, help = "Filter by year (YYYY)")]
    pub year: Option<String>,
}

pub fn build_date_filter_for_date(
    date: &DateRangeFlags,
    current_date: chrono::NaiveDate,
) -> (Option<String>, Option<String>) {
    use chrono::{Datelike, Duration};

    if date.today {
        let day = current_date.format("%Y-%m-%d").to_string();
        return (Some(day.clone()), Some(day));
    }

    if date.yesterday {
        let day = (current_date - Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();
        return (Some(day.clone()), Some(day));
    }

    if date.week {
        let start = current_date - Duration::days(6);
        return (
            Some(start.format("%Y-%m-%d").to_string()),
            Some(current_date.format("%Y-%m-%d").to_string()),
        );
    }

    if date.month {
        let start = current_date.with_day(1).unwrap_or(current_date);
        return (
            Some(start.format("%Y-%m-%d").to_string()),
            Some(current_date.format("%Y-%m-%d").to_string()),
        );
    }

    (date.since.clone(), date.until.clone())
}

pub fn normalize_year_filter(date: &DateRangeFlags) -> Option<String> {
    if date.today || date.yesterday || date.week || date.month {
        None
    } else {
        date.year.clone()
    }
}
