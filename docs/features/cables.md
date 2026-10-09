# Cable and connector calculators

Rust evaluates [the catalog](../../backend/catalog/cables.json); Flutter uses `cablesDiagrams` for static illustrations. No numerical implementation lives in Flutter. Existing calculators remain unchanged.

Source evidence, retrieval limitations and independent example derivations come from [the research handoff](../../research/expansion/cables.md). Source codes S1–S9 below refer to its linked evidence inventory. These are engineering screening models, not product ratings or certification. Defaults and complete expected vectors are retained in the catalog. No build, test, formatter, native rendering or runtime validation was run during this implementation. Parent acceptance must exercise all examples, limiting cases, errors and diagram pixels.

## 1. `cables.twin_lead_geometry` — intermediate

### Physical model and units

S1 gives `q=acosh(D/d)=ln(D/d+sqrt((D/d)²-1))`, `L′=mu0*q/pi`, `C′=pi*eps0*er/q`. Differential impedance is `sqrt(L′/C′)`. Here `D=center_spacing`, `d=wire_diameter`, both in m; er is dimensionless. L′ is the complete differential loop inductance, not the inductance of only one wire. C′ is between the two conductors, not each conductor to ground. `mu0=4*pi*1e-7 H/m`, `eps0=8.8541878128e-12 F/m` are explicit engineering constants.

Domain: d>0, D>d, er>=1; homogeneous nonmagnetic material, straight parallel equal round conductors, negligible conductor/internal-inductance effects and no nearby metal. Do not apply the result exactly to a partly air/partly jacket field or to arbitrary twisted pairs. The catalog avoids `x²-1` cancellation near touching wires: with `u=(D-d)/d`, evaluate `q=log1p(u+sqrt(u)*sqrt((D+d)/d))`.

### Complete worked example

Inputs: `wire_diameter=0.001 m`, `center_spacing=0.006 m`, `relative_permittivity=2.25`.

- `geometry_factor = acosh(6) = 2.477888730288475`.
- `inductance_per_length = 4e-7*q = 9.9115549211539e-7 H/m`.
- `capacitance_per_length = pi*8.8541878128e-12*2.25/q = 2.525802101358135e-11 F/m`.
- `differential_impedance = sqrt(L′/C′) = 198.09396088157135 ohm`.

Independent cases: set er=1 with unchanged geometry: C′ becomes `1.1225787117147267e-11 F/m`, Zdiff becomes `297.140941322357 ohm`, L′ and q stay unchanged. Scaling both lengths equally must leave all outputs unchanged. For `D=d*(1+u)` and small positive u, `q~sqrt(2u)`; verify positive impedance rather than rounded zero. Invalid: D=d, D<d, d=0, er<1. Do not replace the exact acosh geometry by the large-spacing `ln(2D/d)` approximation.

### Mandatory diagram

Cross-section left half: circles centered near (145,135) and (355,135), radius 32. Draw center-spacing dimension between their centers at y=64, and a diameter dimension across one circle at y=195. Put `+V/2` and `-V/2` inside respective circles; dot/cross symbols indicate equal/opposite longitudinal currents. Draw two curved E-field lines from positive to negative conductor; no ground/shield. Right legend near x=440 binds `relative_permittivity` as er, `inductance_per_length` as loop L′, `capacitance_per_length` as pair C′, and `differential_impedance` as Zdiff. Dimension bindings: `center_spacing` and `wire_diameter`. Caption: “Homogeneous balanced round-wire pair. D is center spacing; L′ is loop inductance and C′ is between wires. Field sketch is not to scale.”
