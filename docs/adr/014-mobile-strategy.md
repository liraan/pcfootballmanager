# ADR-014 — Mobile Support Strategy

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Depends on:** ADR-005 (Qt binding), ADR-012 (Plugins)
- **Blocks:** the mobile UI (Phase 3+), the plugin system (mobile constraints)

## Context

The project targets PC first (Windows, Linux, macOS) with a Qt UI (ADR-005). The question is whether — and how — to support Android and iOS.

The core is **mobile-ready by design**: it is pure Rust, has no Qt dependency, and the match engine is textual (low CPU). The `simulate_match` function, the RNG, the save format, and the deterministic rules are platform-agnostic. A phone can run them.

The **UI is not mobile-ready**. The desktop UI is QWidgets (data grids) + QML (skins). QWidgets does not exist on iOS and is limited on Android. QML exists on mobile, but it is a different paradigm (touch, animations, declarative layout). And Qt on iOS raises a **licensing problem**: Qt requires static linking on iOS, and LGPL with static linking is legally contested (the FSF and the Qt Company disagree). Many projects choose the commercial Qt license for iOS, or a non-Qt UI.

The plugin system (ADR-012) is also affected: iOS **forbids** downloading and executing code (App Store Guidelines 2.5.2). A "plugin = Rust crate loaded at runtime" model **cannot exist on iOS** in that form. On Android it is technically possible, but complex (JNI, ABI, sandbox). The pack system (data-only packs) is unaffected.

This ADR decides the strategy. It does **not** implement mobile support: mobile is Phase 3+ (after L1, after the desktop UI). It declares the constraints so that the desktop UI, the plugin system, and the pack system are designed with the mobile target in mind, instead of discovering the incompatibilities later.

## Options considered

### Option A — Qt/QML everywhere (desktop + mobile)
Desktop: QWidgets + QML. Mobile: QML native (Qt for Android/iOS).
- **Pro:** one UI language, one team, one pipeline.
- **Con:** QML is a different paradigm from QWidgets; QWidgets is absent on mobile; Qt on iOS has a licensing problem; `cxx-qt` on mobile is young and undertested.

### Option B — Qt desktop, native mobile UI (CHOSEN)
Desktop: Qt/QML + QWidgets (ADR-005). Mobile: SwiftUI (iOS) / Jetpack Compose (Android) + FFI to the Rust core.
- **Pro:** truly native UI on each platform; Qt remains the desktop UI; Qt licensing is a desktop-only concern; each UI can follow the platform's conventions (touch, sessions, notifications).
- **Con:** two UIs to maintain; two skill sets (or two teams); two FFI layers (Rust↔Swift, Rust↔Kotlin/JNI) to write and test; the plugin system is limited on iOS (see §"Plugin system on mobile").

### Option C — Core Rust + WebView UI on mobile
Desktop: Qt/QML. Mobile: WebView + HTML/JS UI, with the core compiled to WASM.
- **Pro:** one web UI, reusable for debug and mobile; zero Qt licensing on iOS; the core is already Rust.
- **Con:** WebView performance is lower than native; UX is not truly native; WASM has limits (no filesystem, limited threading); the save/load needs platform-specific APIs.

## Decision

**Option B is accepted.**

### 1. The core is mobile-ready, the UI is not

The Rust core (`pcfm-core`, `pcfm-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-match`) is designed to be **platform-agnostic**. It cross-compiles to ARM64 for Android and iOS without changes. The determinism rules apply on every target: the golden tests must run on ARM as well, not only on x86, otherwise "deterministic" means "deterministic on x86".

The UI is **not** mobile-ready. The desktop UI is Qt (ADR-005). The mobile UI is native (SwiftUI / Jetpack Compose), and is a **separate** frontend, not a port of the desktop UI.

### 2. Two UI frontends, one core

```
┌─────────────────────┐     ┌─────────────────────┐
│  Desktop UI (Qt)    │     │  Mobile UI (native) │
│  QWidgets + QML     │     │  SwiftUI / Compose  │
└──────────┬──────────┘     └──────────┬──────────┘
           │                            │
           │  FFI (Rust↔C++)            │  FFI (Rust↔Swift, Rust↔Kotlin)
           │                            │
           └──────────────┬─────────────┘
                          │
              ┌───────────▼───────────┐
              │  Core (Rust, no Qt)   │
              │  pcfm-core, pcfm-*    │
              └───────────────────────┘
```

The FFI layers are **separate crates**: `pcfm-bridge` (Rust↔C++ for Qt, per ADR-004), `pcfm-ffi-swift` (Rust↔Swift), `pcfm-ffi-kotlin` (Rust↔Kotlin/JNI). Each FFI layer follows the rules of ADR-004: thin shim, `catch_unwind` at the boundary, contract tests, no game logic in the shim.

### 3. The plugin system on mobile

ADR-012 defines a plugin as a Rust crate with a `sheet.toml`. On desktop, a plugin can be **loaded at runtime** (dynamic library). On mobile:

- **iOS:** runtime loading is **forbidden** by the App Store. A plugin can only be **compiled into the binary** at build time. There is no "user installs a plugin on iOS".
- **Android:** runtime loading is technically possible, but complex. The default is the same as iOS: **compiled-in plugins**. Runtime loading is an opt-in for advanced users, with the same constraints as the desktop (see ADR-004).

**Consequence for ADR-012:** the plugin system is **build-time on mobile, runtime on desktop**. The plugin sheet, the conflict detection, and the LOD computation are identical in both cases (they operate on the metadata, not on the loading mechanism). Only the loading mechanism differs.

**Consequence for the pack system (L3-packs, planned):** data packs (registry, themes, assets, generated worlds) work on every platform. Code packs (sandboxed DSL, scripts) may be restricted on iOS depending on Apple's review; this is an open question, to be resolved when L3-packs is written.

### 4. Mobile is Phase 3+

Mobile support is **not** in v1.0. The sequence:

1. L0 (done) — match engine, save/load, CLI.
2. L1 — season, world, roster.
3. L2 — career (market, finance, youth, scouting, psyche, crowd, media, AI).
4. Desktop UI (Qt, ADR-005) — the first user-facing UI.
5. L3 — registry, packs, generator, editor, assets, statistics, quality.
6. **Mobile (Phase 3+)** — the native UI frontends, the FFI layers, the build-time plugin model.

Mobile is after the desktop UI is stable, not before. The desktop UI is the reference for the gameplay; the mobile UI adapts it to touch and sessions.

### 5. Determinism on ARM

The golden tests must run on **both x86_64 and ARM64**. A simulation that is deterministic on x86 but not on ARM is not deterministic. The CI must include an ARM runner (or a cross-compilation test with QEMU), and the golden hash must match.

Transcendental functions (`libm`) and floating-point rounding can diverge between architectures. The fixed-point arithmetic for money is unaffected. The `simulate_match` path must be tested for bit-identical output on both architectures.

## Consequences

### Easier
- The core is portable: one Rust codebase, two UIs.
- The desktop UI (Qt) remains the reference; the mobile UI adapts.
- Each UI follows the platform's conventions (touch, sessions, notifications).
- Qt licensing is a desktop-only concern.
- The plugin sheet, the conflict detection, the LOD computation are platform-agnostic.

### Harder
- Two UIs to maintain (Qt + SwiftUI/Compose).
- Two FFI layers to write and test (`pcfm-ffi-swift`, `pcfm-ffi-kotlin`).
- The plugin system is build-time on mobile, runtime on desktop. The model is the same; the loading mechanism differs.
- The golden tests must run on ARM64 (CI complexity).
- The mobile UI must be designed for touch and short sessions, not as a port of the desktop UI.
- The App Store / Play Store review processes add constraints (no runtime code, privacy manifest, etc.).

### Locked in
- Desktop UI: Qt (ADR-005).
- Mobile UI: native (SwiftUI / Jetpack Compose).
- Core: Rust, no Qt, mobile-ready.
- FFI layers: separate crates, ADR-004 rules apply.
- Plugin system: build-time on mobile, runtime on desktop.
- Mobile: Phase 3+, not v1.0.
- Determinism: golden tests on x86_64 and ARM64.

## Open questions

1. **FFI crates.** The exact shape of `pcfm-ffi-swift` and `pcfm-ffi-kotlin` is not defined. They will be defined when the mobile UI is written (Phase 3+).
2. **Code packs on iOS.** Whether the sandboxed DSL is allowed by Apple's review is an open question. It will be resolved when L3-packs is written.
3. **Mobile UX.** The mobile UI design (screens, flows, touch, sessions) is a separate effort, to be specified in a future module (mobile UI is not in the 25 modules; it will be a separate spec, or an extension of `L3-ui`).
4. **Who maintains the mobile UIs.** The project is single-developer today. The mobile UIs may be community-driven, or may require a second maintainer. This is a governance question, not a technical one.
5. **App Store / Play Store.** Distribution, pricing, privacy manifest, and review are not decided. They will be when mobile is closer.

## References

- ADR-004 — Panic strategy at the FFI boundary
- ADR-005 — Qt binding
- ADR-012 — Plugins: The Brick Contract
- ADR-013 — Worlds: LOD, Manifest, and Local Determinism
- `modules/00-foundations.md` §4 — Determinism (golden tests)
- `STRUCTURE.md` §3 — Crate naming
