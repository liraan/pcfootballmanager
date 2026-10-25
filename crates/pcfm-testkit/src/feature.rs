//! Feature identifiers for scenario registration and coverage.

/// A feature of the PCFM framework that a [`Scenario`](crate::Scenario) can
/// exercise.
///
/// The enum is `#[non_exhaustive]` so new variants can be added without
/// breaking downstream code. External plugins should use
/// [`Feature::Plugin`] with their own identifier.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Feature {
    // ── Core primitives ─────────────────────────────────────────────
    CoreRng,
    CoreEventBus,
    CoreIds,
    CoreHash,

    // ── L0 — Match ──────────────────────────────────────────────────
    L0Match,
    L0Save,
    L0GroundTruth,

    // ── L1 — Season / World / Roster ────────────────────────────────
    L1World,
    L1Season,
    L1Roster,

    // ── L2 — Career ─────────────────────────────────────────────────
    L2Market,
    L2Finance,
    L2Youth,
    L2Scouting,
    L2Psyche,
    L2Crowd,
    L2Media,
    L2Ai,

    // ── L3 — Meta ───────────────────────────────────────────────────
    L3Registry,
    L3Packs,
    L3Generator,
    L3Editor,
    L3Assets,
    L3Ui,
    L3Statistics,
    L3Quality,

    // ── External plugin ─────────────────────────────────────────────
    /// A feature contributed by an external plugin (e.g. `"gui"`, `"db"`).
    Plugin(String),
}

impl Feature {
    /// Human-readable label, used in reports.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::CoreRng => "core::rng".into(),
            Self::CoreEventBus => "core::event_bus".into(),
            Self::CoreIds => "core::ids".into(),
            Self::CoreHash => "core::hash".into(),
            Self::L0Match => "L0::match".into(),
            Self::L0Save => "L0::save".into(),
            Self::L0GroundTruth => "L0::ground_truth".into(),
            Self::L1World => "L1::world".into(),
            Self::L1Season => "L1::season".into(),
            Self::L1Roster => "L1::roster".into(),
            Self::L2Market => "L2::market".into(),
            Self::L2Finance => "L2::finance".into(),
            Self::L2Youth => "L2::youth".into(),
            Self::L2Scouting => "L2::scouting".into(),
            Self::L2Psyche => "L2::psyche".into(),
            Self::L2Crowd => "L2::crowd".into(),
            Self::L2Media => "L2::media".into(),
            Self::L2Ai => "L2::ai".into(),
            Self::L3Registry => "L3::registry".into(),
            Self::L3Packs => "L3::packs".into(),
            Self::L3Generator => "L3::generator".into(),
            Self::L3Editor => "L3::editor".into(),
            Self::L3Assets => "L3::assets".into(),
            Self::L3Ui => "L3::ui".into(),
            Self::L3Statistics => "L3::statistics".into(),
            Self::L3Quality => "L3::quality".into(),
            Self::Plugin(name) => format!("plugin::{name}"),
        }
    }
}
