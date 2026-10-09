# Copper — electronics design workbench

Flutter desktop-only frontend and a private Rust calculation process. All **128 calculators** are implemented: 39 basic, 50 intermediate and 39 expert across 16 electronics domains. Forms, units, formulas, model limits, cited sources and independently worked examples come from the embedded Rust catalog; Flutter does not duplicate the mathematics. Calculations run offline, without an HTTP service.

## Research and delivery artifacts

- [Common 128-feature board](../RESEARCH_BOARD.md): reserved boundaries, complete ID index and per-lane research.
- [Markdown communication bus](../COMMUNICATION.md): addressed research decisions, Git handoffs and QA findings.
- [Scrum-master sprint](../coordination/SPRINT.md) and [complete acceptance backlog](../coordination/BACKLOG.md).
- [Architecture](../coordination/ARCHITECTURE.md) and [catalog/protocol contract](../coordination/CONTRACT.md).
- `../coordination/developer_handoffs.json`, `../coordination/testing/*.json`: actual independent agent handoffs.
- `../coordination/reviews/*.md`: frozen baseline reviews; `../coordination/reviews/closure.json`: post-fix independent dispositions and attribution of executed checks.
- `../coordination/numerical*.json`: actual before/after Rust-process counterexamples, with independent expected values after correction.

The pipeline used 16 researchers, one scrum-master, 16 developers and then 16 testers plus 16 reviewers concurrently. Developers own eight distinct features each in isolated `feat/<lane>` worktrees; each feature has its own implementation commit. Testers own independent `test/<lane>` worktrees. Only the integration owner cherry-picks explicit handoff SHAs, changes shared Rust/Flutter infrastructure and executes global checks. Maximum concurrent worker wave: 32. No remote repository was supplied.

## Prerequisites

Verified on Linux with Rust/Cargo **1.98.1**, Flutter **3.47.7**, Dart **3.13.5**. The Rust package uses edition 2024. Put `cargo`, `flutter` and `dart` on PATH. Linux desktop builds require Clang, CMake, Ninja, pkg-config and GTK 3 development libraries. Windows requires its native Flutter/Visual Studio C++ desktop toolchain; macOS requires its native Flutter/Xcode toolchain. Building dependencies may need network access; running calculators does not.

## Development

From the repository root:

```sh
cargo build --locked --manifest-path backend/Cargo.toml
```

Then from `frontend/`:

```sh
flutter pub get
PCB_CALCULATOR_BACKEND=/absolute/path/to/backend/target/debug/pcb-calculator flutter run -d linux
```

`PCB_CALCULATOR_BACKEND` must name the native executable, not a directory. On Windows the executable is `pcb-calculator.exe`; set the same environment variable using PowerShell before `flutter run -d windows`. On macOS use `flutter run -d macos`. Without an override the app looks beside its own native executable, including inside `Contents/MacOS` on macOS. Missing engines and numerical/model errors are shown explicitly; no fake result or remote fallback is used.

## Native release bundle

From `frontend/`:

```sh
dart run tool/build_desktop.dart
```

This builds Rust and Flutter in release mode for the host desktop and copies the Rust executable beside the native application. Linux x64 output is `build/linux/x64/release/bundle/`; keep the complete directory together, including `lib/`, `data/` and `pcb-calculator`. Windows output is `build/windows/x64/runner/Release/`; macOS output is `build/macos/Build/Products/Release/pcb_design_calculator.app`.

Linux launch, without a development override:

```sh
./build/linux/x64/release/bundle/pcb_design_calculator
```

No Android, iOS or web runner is included. Native Windows/macOS builds and runtime acceptance cannot be inferred from Linux results; the CI matrix is configured for all three but was not remotely executed in this local-only workflow. Distribution signing/notarization is not certified.

## Verification

From the repository root:

```sh
cargo test --locked --manifest-path backend/Cargo.toml
cargo build --locked --manifest-path backend/Cargo.toml
python3 tools/smoke_backend.py
```

From `frontend/` on a graphical Linux desktop:

```sh
flutter analyze
PCB_CALCULATOR_BACKEND=/absolute/path/to/backend/target/debug/pcb-calculator flutter test integration_test/workbench_test.dart -d linux
```

The Rust suite covers every researched example, independent identities, meaningful input/model boundaries, signed/zero values, strict small-value precision and error recovery. The process smoke checks all 128 complete example output maps against researched values. The native integration journey checks basic/intermediate/expert examples, error presentation and immediate stale-result invalidation. Integration keyboard injection is explicitly registered so native IME updates do not overwrite test input. CI is at `../.github/workflows/desktop.yml`.

Known dependency risk: transitive `nom 1.2.4` emits Cargo's future-incompatibility warning on Rust 1.98.1. Current checks pass; this is not a guarantee for a future compiler. The warning is not suppressed.

## Engineering limits

These are closed-form models and engineering estimates, not a SPICE simulator, standards-compliance determination, hardware-safety approval or substitute for manufacturer specifications. Respect each calculator's topology, sign conventions, empirical assumptions and admissible numerical range. Finite-input validation and checked intermediate arithmetic reject invalid computations rather than manufacture plausible zeros. Subnormal outputs retain explicit floating-point/ULP limitations.
