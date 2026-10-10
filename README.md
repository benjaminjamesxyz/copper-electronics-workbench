# Copper — electronics design workbench

Copper is an offline desktop application for hardware and electronics design calculations. It ships **228 calculators** across 34 domains — DC and AC networks, power electronics, RF and signal integrity, magnetics, thermal design, batteries, motors, protection, EMC, metrology and more. Every calculator provides labeled inputs with units and valid ranges, a worked example with independently derived expected results, the formula used, stated model assumptions, and cited primary sources.

The mathematics lives in a private **Rust engine**; the **Flutter** frontend is desktop-only (Linux, Windows, macOS) and renders no numbers itself. Calculations run entirely on your machine — no network, no cloud service, no simulator.

Each calculator is illustrated with a native vector diagram — a circuit schematic, physical cross-section, timing diagram or energy-flow sketch — whose quantity labels bind to the exact input and output names on the form.

## Feature summary

| Area | Examples |
|---|---|
| DC & AC circuits | Ohm's law, loaded dividers, Wheatstone bridges, resonance, power factor correction |
| Power & converters | Buck/boost sizing, SEPIC, forward/flyback/LLC/DAB, charge pumps |
| RF & signals | L-match, attenuators, link budgets, Fresnel clearance, crosstalk, jitter SNR |
| Magnetics & cores | Transformer turns, toroid/gapped inductors, Steinmetz loss, foil windings |
| Semiconductors | Diode recovery, BJT/MOSFET biasing, op-amp stages, instrumentation front ends |
| Batteries, motors & protection | Peukert runtime, KiBaM recovery, PMSM dq operating points, fuse/TVS/MOV coordination |
| Sensors, measurement & precision | NTC/Callendar–Van Dusen, Hall/AMR, ADC metrics, Howland current sources, LOG114 |
| PCB, thermal, EMC, logic | Trace heating, shielding, snubbers, noise margins, metastability, FIFO sizing |

Levels range from basic to expert; every model states its assumptions and limits. Engineering estimates — not certification, not a SPICE substitute.

## Quick start

Prerequisites on Linux: **Rust/Cargo** (edition 2024), **Flutter** with the Linux desktop toolchain (Clang, CMake, Ninja, pkg-config, GTK3 dev libraries). Windows and macOS need their native Flutter desktop toolchains.

```sh
git clone git@github.com:benjaminjamesxyz/copper-electronics-workbench.git
cd copper-electronics-workbench
```

### Run (development)

```sh
cargo build --locked --release --manifest-path backend/Cargo.toml
cd frontend
flutter pub get
PCB_CALCULATOR_BACKEND=$PWD/../backend/target/release/pcb-calculator flutter run -d linux
```

`PCB_CALCULATOR_BACKEND` must name the engine **executable**, not a directory. Without it, the app looks for the engine beside its own binary — which is how the packaged bundle works.

### Run (packaged release)

```sh
cd frontend
dart run tool/build_desktop.dart        # builds Rust + Flutter, copies the engine into the bundle
./build/linux/x64/release/bundle/pcb_design_calculator
```

Keep the bundle directory together (`lib/`, `data/`, `pcb-calculator`). Development override unset — the packaged app is self-contained and fully offline.

### Verify

```sh
cargo test --locked --release --manifest-path backend/Cargo.toml   # full numerical suite (856 tests)
python3 tools/smoke_backend.py backend/target/release/pcb-calculator  # all 228 examples through the real process
python3 tools/replay_scenarios.py                                  # 481 ordered lane scenarios
cd frontend && flutter test test/diagrams_test.dart                # layout, large-text and accessibility coverage
```

## Using the app

1. **Search or filter** the calculation library (left panel) by name, domain or experience level.
2. **Select** a calculator: its illustration, input form, formula and references appear in the detail pane.
3. **Enter inputs** with units and helper ranges shown; press **Calculate** (or Enter) — results appear as cards with readable scientific formatting.
4. **Load worked example** fills a validated reference case whose expected outputs are shown under *Worked example & references*.
5. Editing any input immediately invalidates stale results; model-domain violations show an explicit error — never a plausible-looking zero.

## Bundle layout (Linux)

```
frontend/build/linux/x64/release/bundle/
├── pcb_design_calculator   # Flutter desktop binary
├── pcb-calculator          # Rust engine, found automatically at startup
├── data/                   # Flutter assets
└── lib/                    # shared libraries
```

## Documentation

- [`frontend/README.md`](frontend/README.md) — architecture, engine protocol, packaging, CI and verification details.
- [`docs/features/`](docs/features/) — per-domain model documentation: derivations, sign conventions, boundary behavior and worked examples for all 228 calculators.
- [`docs/INDEX.md`](docs/INDEX.md) — index routing you to the right model documentation.

## Known limitations

- Closed-form engineering estimates with stated assumptions — **not** certification, standards compliance, a SPICE simulator, or a substitute for manufacturer data sheets.
- Each model documents its valid domain; out-of-domain inputs are rejected with explicit errors rather than coerced.
- Linux is the verified platform; Windows/macOS builds are configured but not exercised in this repository's CI runs.
- The transitive Rust dependency `nom 1.2.4` emits a future-incompatibility warning on current compilers (visible, unsuppressed, harmless today).

## License and support

See the repository for current licensing. Calculators are provided for engineering study and early design trade-offs; always validate critical designs against manufacturer specifications and appropriate standards.
