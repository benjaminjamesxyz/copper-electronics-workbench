# Model documentation index

Each file documents one domain of the calculator catalog: physical model, derivations, sign conventions, valid input domains, boundary behavior and worked examples. File names match the calculator ID prefix (for example, the `ac.*` calculators are documented in [ac.md](ac.md)).

| Domain | File | Calculators |
|---|---|---|
| AC impedance & resonance | [ac.md](ac.md) | `ac.*` |
| Batteries | [batteries.md](batteries.md) | `batteries.*` |
| Bipolar junction transistors | [bjt.md](bjt.md) | `bjt.*` |
| Cables & connectors | [cables.md](cables.md) | `cables.*` |
| Control loops | [control.md](control.md) | `control.*` |
| Power converters | [converters.md](converters.md) | `converters.*` |
| Magnetic cores | [cores.md](cores.md) | `cores.*` |
| Digital & data conversion | [data.md](data.md) | `data.*` |
| DC resistor networks | [dc.md](dc.md) | `dc.*` |
| Diodes | [diodes.md](diodes.md) | `diodes.*` |
| EMC & shielding | [emc.md](emc.md) | `emc.*` |
| Filters | [filters.md](filters.md) | `filters.*` |
| Digital logic design | [logic.md](logic.md) | `logic.*` |
| Magnetics & transformers | [magnetics.md](magnetics.md) | `magnetics.*` |
| Measurement & metrology | [measurement.md](measurement.md) | `measurement.*` |
| MOSFETs | [mosfet.md](mosfet.md) | `mosfet.*` |
| Motors & actuators | [motors.md](motors.md) | `motors.*` |
| Noise | [noise.md](noise.md) | `noise.*` |
| Op-amps | [opamps.md](opamps.md) | `opamps.*` |
| Optics & photodetection | [opto.md](opto.md) | `opto.*` |
| Oscillators | [oscillators.md](oscillators.md) | `oscillators.*` |
| Passive components | [passives.md](passives.md) | `passives.*` |
| PCB design | [pcb.md](pcb.md) | `pcb.*` |
| Power electronics | [power.md](power.md) | `power.*` |
| Precision analog | [precision.md](precision.md) | `precision.*` |
| Protection | [protection.md](protection.md) | `protection.*` |
| RF design | [rf.md](rf.md) | `rf.*` |
| Sensors | [sensors.md](sensors.md) | `sensors.*` |
| Signal integrity | [signals.md](signals.md) | `signals.*` |
| Thermal design | [thermal.md](thermal.md) | `thermal.*` |
| Timing & interfaces | [timing.md](timing.md) | `timing.*` |
| Transients | [transients.md](transients.md) | `transients.*` |

These documents describe model intent, assumptions and limits. The catalog definitions themselves live in [`backend/catalog/`](../backend/catalog/) and are the executable source of truth; the desktop app displays the same formulas, assumptions and references at run time.

See also: [project README](../README.md) for build and run instructions.
