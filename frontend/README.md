# Copper — electronics design workbench

Flutter desktop-only frontend and a private Rust calculation process. All **228 calculators** are implemented: 65 basic, 89 intermediate and 74 expert across 34 selectable domains. Forms, units, formulas, model limits, cited sources and independently worked examples come from the embedded Rust catalog; Flutter does not duplicate the mathematics. Calculations run offline, without an HTTP service.

Every calculator also has a labeled vector illustration: a circuit, physical cross-section, signal response, timing diagram or thermal/energy-flow sketch. These render offline with native Flutter drawing; no image service, browser or additional dependency is required.

## Research and delivery artifacts

- [Common feature board](../RESEARCH_BOARD.md): original 128 features, 100 additional features, reserved boundaries, complete ID indices and per-lane research.
- [Markdown communication bus](../COMMUNICATION.md): addressed research decisions, Git handoffs and QA findings.
- [Expansion scrum-master sprint](../coordination/expansion/SPRINT.md) and [100-item acceptance backlog](../coordination/expansion/BACKLOG.md); [original sprint](../coordination/SPRINT.md) and [128-item backlog](../coordination/BACKLOG.md).
- [Architecture](../coordination/ARCHITECTURE.md) and [catalog/protocol contract](../coordination/CONTRACT.md).
- `../coordination/developer_handoffs.json`, `../coordination/testing/*.json`: actual independent agent handoffs.
- `../coordination/reviews/*.md`: frozen baseline reviews; `../coordination/reviews/closure.json`: post-fix independent dispositions and attribution of executed checks.
- `../coordination/numerical*.json`: actual before/after Rust-process counterexamples, with independent expected values after correction.
- `../coordination/expansion/developer_handoffs.json`: sixteen expansion developers' original and integrated feature commits; `../coordination/expansion/process_examples.json`: complete observed outputs for all 228 examples.

Each sprint uses 16 researchers, one scrum master, 16 developers and then 16 testers plus 16 reviewers concurrently. Original developers owned eight features each; expansion developers own six or seven each. Expansion branches use isolated `feat/expand-<lane>` and `test/expand-<lane>` worktrees, with one implementation commit per calculator including its mandatory diagram and documentation. Only the integration owner cherry-picks explicit handoff SHAs, changes shared Rust/Flutter infrastructure and executes global checks. Maximum concurrent worker wave: 32. No remote repository was supplied.

## Reading the illustrations

- The illustration precedes the inputs. Its caption explains topology, reference directions, geometry and model assumptions.
- The legend connects each annotated quantity to the existing form/result name, unit and input/output role. A derived annotation such as `Ts = 1/fs` references the frequency parameter; it is not a displayed numerical result.
- Current arrows define positive reference directions. Circuit junctions have dots; ordinary wire crossings without a dot are not joined. Physical outlines and dimension arrows are geometry, not electrical connections.
- Curves and dimensions are explanatory, **not live simulations or scale drawings**. Changing inputs changes numerical results, not the illustration's geometry.
- Narrow windows retain readable symbol sizes and show an interactive horizontal scrollbar. Desktop text scaling enlarges the vector canvas. Captions and quantity descriptions provide a text alternative to the drawing.

Illustrations are authored in 32 `lib/diagrams/<domain>.dart` files, indexed by exact catalog ID in `catalog.dart`. Shared symbols, connector conventions, painter and panel are adjacent. The original illustration pass used sixteen independent `diagram/<domain>` worktrees; the expansion delivers each new illustration together with its numerical model. Neither pass adds presentation geometry to the Rust process contract.

The historical Linux illustration gate rendered the original 128 diagrams and cleared sixteen source/pixel review lanes. Its [evidence](../coordination/diagram_evidence.json), [review closure](../coordination/diagram_reviews.json) and [packaged circuit view](../coordination/desktop-illustrations-circuit.png) describe baseline `29f9c2b`, not expansion acceptance. Permanent catalog-coverage and accessible-layout checks remain in `test/diagrams_test.dart`.

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

Historical acceptance before this expansion passed 431 Rust tests, all 128 complete process examples, 27 numerical review cases and native UI selection/metadata for all 128 IDs. Its extracted archive launched without an engine override, computed a 32-bit ADC through mouse/keyboard input, and closed normally. [`../coordination/desktop_evidence.json`](../coordination/desktop_evidence.json) retains that earlier evidence; it is not proof for the expanded release. The generated `copper-Linux.tar.gz` contains a complete bundle and is ignored by Git.

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
flutter test test/diagrams_test.dart
PCB_CALCULATOR_BACKEND=/absolute/path/to/backend/target/debug/pcb-calculator flutter test integration_test/workbench_test.dart -d linux
```

The Rust suite covers researched examples, independent identities, meaningful input/model boundaries, signed/zero values, strict small-value precision and error recovery. The process smoke checks all 228 complete example output maps against researched values. The native integration journey checks basic/intermediate/expert examples, error presentation, immediate stale-result invalidation and exact 32-bit ADC code-count display. Integration keyboard injection is explicitly registered so native IME updates do not overwrite test input. CI is at `../.github/workflows/desktop.yml`.

Known dependency risk: transitive `nom 1.2.4` emits Cargo's future-incompatibility warning on Rust 1.98.1. Current checks pass; this is not a guarantee for a future compiler. The warning is not suppressed.

## Engineering limits

These are closed-form models and engineering estimates, not a SPICE simulator, standards-compliance determination, hardware-safety approval or substitute for manufacturer specifications. Respect each calculator's topology, sign conventions, empirical assumptions and admissible numerical range. Finite-input validation and checked intermediate arithmetic reject invalid computations rather than manufacture plausible zeros. Subnormal outputs retain explicit floating-point/ULP limitations.

Result formatting retains exactly representable large integers through the IEEE-754 safe-integer boundary; continuous quantities use seven significant digits. The underlying Rust result remains a finite `f64`, not an arbitrary-precision value.
