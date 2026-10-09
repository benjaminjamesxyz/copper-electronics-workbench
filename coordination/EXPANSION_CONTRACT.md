# Second calculation sprint: 100 additions with mandatory diagrams

## Goal and baseline
Add exactly 100 genuinely new hardware-design calculators to the 128 delivered at `29f9c2b`, for 228 total. Preserve every existing ID, model, example and diagram. Rust owns numerical calculations; Flutter remains desktop-only. Existing schema, engine, validation, painter and layout stay authoritative. `coordination/CONTRACT.md` defines the numerical schema; its eight-per-lane quota belongs to the first sprint and does not apply here.

The live baseline was read from the actual Rust release process. `coordination/expansion/baseline.json` records all 128 IDs, physical models, inputs and outputs. Read that exclusion inventory and `RESEARCH_BOARD.md` before selecting features. Different inverses, renamed formulas, extra outputs, units or ordinary input variations are not additional calculations. Every proposed item must name its nearest existing/neighboring calculation and explain a distinct physical design problem/model.

## Reserved research and implementation lanes
| Prefix | Count | Owned problem boundary | Exclusions / adjacent owners |
|---|---:|---|---|
| sensors | 7 | Electrical transduction: thermistors, nonlinear RTDs, strain/piezo/Hall/temperature sensor models | Generic resistor TCR and Wheatstone solution already exist; precision owns amplifier stages; opto owns optical transducers |
| opto | 7 | Photodetection, illumination, optical propagation and fiber-link hardware | Exclude generic LED ballast, shot noise and amplifier design; precision owns transimpedance electronics |
| oscillators | 7 | Timer, relaxation, RC/LC and crystal oscillator design | Exclude plain RC delay, timer division, passive RLC resonance and ordinary filter response |
| rf | 7 | RF matching/attenuation networks, antennas, RF link/intermodulation design | Existing resistive reflections, coax impedance, quarter-wave matching and Friis noise cascade excluded; measurement owns jitter/metrology |
| batteries | 6 | Battery packs, charge/discharge state and nonlinear battery limits | Exclude mere Ohm law, generic conversion efficiency and capacitor energy; protection owns fault devices |
| protection | 6 | Fuse, transient clamp, crowbar and protection coordination models | Exclude existing inductive spike, diode recovery and ordinary RC charging; emc owns snubbers/EMI filtering |
| converters | 6 | Additional power-converter topologies and topology-specific operating constraints | Buck/boost sizing and LDO loss already exist; control owns compensators and small-signal loop design |
| control | 6 | Power/control feedback design and stability models | Exclude existing amplifier GBW, generic RLC step and ordinary filter synthesis; converters owns steady converter design |
| emc | 6 | Shielding, interference conversion, conducted EMI and suppression design | Exclude plate displacement current, lumped capacitive crosstalk and skin depth; rf owns intentional antennas/links |
| cables | 6 | Cable/connectors, distributed lossy interconnect and distinct line geometries | Existing microstrip/stripline/coax, simple propagation/reflections and PCB DC copper models excluded; cores owns coils |
| precision | 6 | Precision analog front ends, instrumentation, transimpedance, references and current sources | Existing eight op-amp stages and generic amplifier-noise budgets excluded; sensors/opto own physical transduction |
| logic | 6 | Digital hardware logic levels, loading, synchronization and interface design | Existing I2C pullup, UART mismatch, setup slack, timer divider and Schmitt RC delay excluded; measurement owns clock jitter |
| motors | 6 | Electromechanical motor/actuator/drive models and energy exchange | Exclude mere DC resistance, RL buildup and generic switching loss; must include electromechanical behavior |
| passives | 6 | Nonideal capacitor/resistor/component-network design and component characteristics | Exclude existing ideal RC/RLC response, generic TCR/derating and trace loss; cores owns all magnetic components |
| cores | 6 | Additional coil geometry, winding construction and magnetic loss/design models | Existing toroid/gapped-core inductance, turns ratio, sinusoidal flux, coupling and skin depth excluded |
| measurement | 6 | Hardware measurement loading, uncertainty, sampling aperture/jitter and instrument response | Exclude existing ADC quantization/ENOB/alias/settling and generic noise sums; logic owns digital interfaces |

Each lane must include basic, intermediate and expert models. These are reservations, not claims that every candidate passes novelty or physical review. Resolve collisions before promotion; do not pad counts with duplicates.

## Research deliverable — sixteen parallel researchers
Own only `research/expansion/<lane>.json` and `.md`, plus append-only bus messages. JSON is a complete array of the reserved count using the existing numerical schema, ready for implementation. Markdown must contain for every ID:
- Physical purpose and novelty comparison against the closest old/neighbor model.
- Primary-source links with source type/section, equations, units, sign convention, model validity and limitations. Actually read cited material; identify inaccessible sources rather than pretending verification.
- Complete independent worked example for every output. Use explicit numeric values; derive them outside the Rust engine. All defaults/examples must be physically valid.
- Meaningful nominal, limiting and invalid-domain cases for an independent tester.
- **Mandatory diagram specification:** circuit topology, physical geometry, waveform or energy/control flow; exact catalog quantity keys; labels/polarities/directions; explanatory caption. Not a generic domain decoration.
Record search boundaries and evidence gaps concisely. Use the established closed-form engine and stable algebra (`expm1`, `log1p`, `hypot`, etc.). Do not invent formula language features, use reserved variable keys, hide unsupported domains or impose arbitrary bounds merely to make tests pass.

## Sprint and Git gates
1. Parent checks sixteen research deliveries for exactly 100 unique and genuinely additional models and publishes common-board reservations.
2. Only after research completes, a scrum-master agent plans the sprint/backlog. All 100 items and their diagrams are required, never deferred.
3. Sixteen developers run concurrently in `.worktrees/expand-dev-<lane>`, branches `feat/expand-<lane>`. Own only `backend/catalog/<lane>.json`, `frontend/lib/diagrams/<lane>.dart`, and `docs/features/<lane>.md`. Each feature has one accountable lane owner; commit each completed calculator with its own diagram/docs together. Parent owns the shared diagram registry and any necessary engine integration.
4. Parent integrates explicit developer commit lists into `integration/electronics-calculators`. No force push, shared checkout switching or remote assumptions.
5. Sixteen testers use `.worktrees/expand-test-<lane>`, branches `test/expand-<lane>`, owning `backend/tests/<lane>.rs` and `coordination/expansion/testing/<lane>.json`. Sixteen reviewers independently assess the integrated implementation at the same gate. Maximum 32 simultaneous workers.
6. Parent integrates tests, runs actual checks and desktop journeys, fixes all actionable findings, and returns changed models/images for review closure before delivery.

All workers skip builds, tests, lint and formatters. Testers write substantive independent assertions and provide scenarios; parent executes the complete wave once. A failed check gets a root-cause fix and focused confirmation. No source-text tests, wiring checks, weak finite/nonempty assertions or implementation-default snapshots.

## Mandatory illustration contract
Use `frontend/lib/diagrams/primitives.dart` exactly. Export `const <lane>Diagrams = <String, CalculatorDiagram>{...};`, one entry per new ID; import only `primitives.dart`. Parent updates the shared registry. Reuse the painter and accessible panel; no new package, image service or duplicate framework.
- Canvas 700x280; leave 16px margins, use labels at least 14px, reserve clear space between text and geometry.
- `Label.quantity` must exactly name an input/output. Derived labels must explain their relation, never imply a different quantity equals the bound value.
- Respect documented component pins, positive/reference direction and electrical connectivity. Break wires at component bodies; crossing wires only connect with explicit junctions.
- Diagram must teach this specific physical model. Captions state idealizations, scale/plot limits and nonobvious topology.
- Static explanatory curves are not computed results. Do not fake live plots or show numerical output without computing it.
- Parent captures actual native images for every new ID. Reviewers inspect the pixels as well as the model/source. Existing diagram coverage and large-text/narrow-window behavior remain required.

## Communication bus and ownership
The shared bus remains `/home/simson/pi-projects/pcb-design-calculator/COMMUNICATION.md`, even when working in an isolated worktree. Read its expansion entries before choosing models, before handoff and whenever an overlap is found. Append concise addressed findings/claims/handoffs with `read` plus anchored `edit`; stale hashes require rereading and retrying. Never rewrite other entries. Immediate agent messages may supplement but do not replace the Markdown record. Parent alone updates the common board's allocation/index/status and shared integration files.

## Acceptance
All 100 new IDs have correct implemented numerical models, source-backed limits, independent numerical/boundary tests, explanatory diagrams and closed reviews. Existing 128 remain unchanged and functional. Exercise the actual Rust process with every complete example; traverse all 228 in the real Flutter desktop and calculate representative/new examples through its native UI. Inspect every new diagram image, narrow scrolling and a packaged release launched without an override. Report only observed host/platform checks, and retain all review/evidence records. Windows/macOS and remote CI are not implied by Linux success. Engineering estimates are not certification or substitutes for manufacturer specifications or simulation.
