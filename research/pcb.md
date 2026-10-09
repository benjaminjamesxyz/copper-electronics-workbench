# PCB interconnect and manufacturing research

Exactly eight calculations are defined in [pcb.json](pcb.json). The catalog is for offline closed-form engineering estimates, not certification or simulation. Levels span basic (2), intermediate (3), and expert (3). Constraints use only input keys/constants; output expressions may reference preceding outputs. Every numerical example supplies every input and every output expected by the implementation contract.

## Scope, search and evidence quality

Research covered publicly accessible English manufacturer application notes, a PCB fabricator's process definitions, university teaching material, NIST unit tables, and the public IPC-2152 scope/table of contents. No publication-date restriction was imposed: geometric and physical laws do not depend on publication age, while process-dependent coefficients and old manufacturing examples are explicitly qualified. Searches included PCB trace/via resistivity, the IPC-2221 coefficients and their limitations, copper-weight/thickness conventions, differential length skew, annular-ring tool size, Joule heat density, and stencil aperture area ratio. Generic hobby calculators, uncited numerical assertions, RF-impedance models, and inverse rearrangements as separate calculators were excluded.

Sources were fetched with web search and static-document reads. TI's original documents, JLCPCB's guide, Eurocircuits' annular-ring explanation, Michigan Tech's Joule-heating material, and NIST's tables were directly readable. Some Eurocircuits copper/stencil pages returned only navigation, so they are **not** used as evidence for their missing body text. Copper.org material-property pages and the IPC-2152 PDF could not be directly fetched; Copper.org values were therefore excluded. IPC-2152 is cited only for its public scope/table-of-contents search excerpt and to identify the more detailed standard, not as a claim that the full controlling standard was inspected. TI's readable equation is the direct numerical source for the legacy current fit.

### Source register

| Key | Source and actual URL | Evidence weight and limitation |
|---|---|---|
| TI-R | [TI, Robert Kollman, Constructing Your Power Supply—Layout Considerations, Topic 4, section II, Tables 1–2 and Figure 3](https://www.ti.com/lit/pdf/slup224) | Primary manufacturer engineering guidance (2004). Supports conductor geometry, resistance, plated-via treatment, and example resistivities. The plated-copper value is an illustrative process value, not a current universal material specification. |
| TI-I | [TI TIDA-01232, High-Current, Low-Noise Parallel LDO Reference Design, section 3.1.1, Equations 6–7](https://www.ti.com/lit/ug/tiducs7/tiducs7.pdf) | Primary manufacturer design implementation (2017). Equation 7 explicitly identifies square-mil area, exponents 0.44/0.725, and 0.048/0.024 coefficients. A reproduction/application of IPC-2221, not the controlling IPC publication. |
| IPC-S | [IPC-2152 public scope and table of contents](https://www.ipc.org/TOC/IPC-2152.pdf) | Standards-body public outline, accessed through search excerpts; direct PDF fetch failed. Establishes the separate board-construction-aware current-capacity topic, not numerical coefficients or compliance limits. |
| JLC-C | [JLCPCB Copper Weight (Thickness) Guide](https://jlcpcb.com/help/article/jlcpcb-copper-weight) | Primary fabricator definition, updated September 2026. Explicit nominal 1 oz = 35 µm convention; actual supplier/order capabilities can change. |
| NIST-U | [NIST Handbook 133 (2026), Appendix E, General Tables of Units](https://www.nist.gov/document/2026-hb-133-appendix-e) | Primary national-metrology reference. Exact international inch/foot conversions; not a copper-thickness manufacturing specification. |
| TI-S | [TI Clock Conditioner Owner's Manual, SNAA103, Chapter 6, Minimizing Skew](https://www.ti.com/lit/pdf/snaa103) | Primary manufacturer teaching/design material. Directly relates path-length mismatch to skew and gives 6.84 ps/mm as an approximate FR-4 scale; not a universal laminate specification. |
| TI-D | [TI SPRACP4A, Jacinto7 AM6x/TDA4x/DRA8x High-Speed Interface Design Guidelines, section 2.3](https://www.ti.com/lit/an/spracp4a/spracp4a.pdf) | Primary manufacturer guidance, revised June 2024. Defines intra-pair versus inter-pair matching and cautions about practical routing; not an all-interface skew limit. |
| EC-A | [Eurocircuits, What is the Inner Annular Ring (IAR)?](https://www.eurocircuits.com/iar-inner-annular-ring/) | Primary fabricator geometry/process definition. Explicit production tool-size calculation and off-center hole discussion; example drill allowance belongs to this supplier. |
| MTU-J | [Michigan Technological University, Thermoelectromagnetics: Joule Heating](https://multiphysics.geo.mtu.edu/thermoelectromagnetics.html) | University explanatory physics source. Directly derives volumetric heating as J·E = rho J²; no PCB thermal qualification data. |
| TI-A | [TI SPRAAV2A, PCB Assembly Guidelines for 0.4 mm PoP Packages, Part II, section 6.1](https://www.ti.com/lit/an/spraav2a/spraav2a.pdf) | Primary manufacturer assembly guidance (2013). Defines aperture-opening/wall area ratio and gives a 0.66 guideline for round fine-pitch BGA apertures. Rectangular geometry is independently derived below; transfer performance remains process dependent. |

The first-principles resistance, annular geometry, skew conversion, and Joule-generation laws have strong mathematical support **within the stated idealizations**. The IPC estimate has only screening-level confidence for a real board. Nominal copper weight is a convention, not a measured thickness. Stencil area ratio is exact geometric arithmetic but only a qualified process-screening predictor. Sources sharing TI authorship are not presented as independent experimental validations of each other.

## Ownership and distinctness

| ID | Distinct hardware question | Boundary |
|---|---|---|
| `pcb.trace_resistance_drop` | How much one-way DC interconnect resistance/drop does a finished rectangular trace add? | Geometry-driven parasitic resistance; not generic Ohm-law current or resistor power. |
| `pcb.ipc2221_current_estimate` | What legacy steady-current screening estimate corresponds to an area and selected temperature rise? | Empirical electrothermal correlation; not an inverse of trace resistance or an IPC-2152 solver. |
| `pcb.via_barrel_resistance` | What axial resistance does a plated cylindrical layer transition add? | Exact annular barrel geometry, process resistivity and ideal parallel bank; no via ampacity claim. |
| `pcb.nominal_copper_thickness` | What nominal thickness does a manufacturing copper-weight designation imply? | Manufacturing convention; not finished measured copper or trace-current synthesis. |
| `pcb.differential_length_skew` | What intra-pair arrival mismatch comes from routed length mismatch? | Two-leg mismatch only, with supplied common delay density; absolute line propagation and RF impedance remain in signals. |
| `pcb.annular_ring_registration` | What copper ring remains after production drilling and registration offset? | Fabrication geometry and tolerances; not via electrical resistance. |
| `pcb.trace_joule_heat_density` | Where/how rapidly is heat generated in copper, and how much energy is generated during a rectangular pulse? | Local current/heat density and deposited energy; no conductor temperature or generic resistor-power-only calculator. |
| `pcb.stencil_aperture_area_ratio` | How does stencil opening area compare with paste-contacting aperture-wall area? | Assembly manufacturability; independent of copper conduction and hole annular ring. |

The shared communication bus records these boundaries. Trace loss generation deliberately does not compute temperature, thermal resistance, thermal transients or resistor TCR; those remain thermal-lane responsibilities. The heat calculator includes power only as an intermediate integral of local volumetric generation, not as an additional counted problem. No formula is counted again solely because it can be rearranged.

## 1. Finished trace DC resistance and drop — basic

**ID:** `pcb.trace_resistance_drop`

**References:** [TI-R](https://www.ti.com/lit/pdf/slup224), Topic 4, section II and Table 1.

For a uniform trace of length L, finished width w and finished thickness t, the conducting cross-section is A = wt. Local Ohm's law gives electric field E = rho J, and uniform current density is J = I/A. Integrating the axial field over L gives V = rho IL/A. Thus:

- `cross_section = width*thickness`, in m²;
- `resistance = resistivity*length/cross_section`, in ohms;
- `voltage_drop = current*resistance`, in V.

Inputs L, w and t are metres; rho is ohm·m at the actual conductor temperature; I is nonnegative DC current magnitude in amperes. The unit check is `(ohm m)*m/m² = ohm`, then `A*ohm = V`.

**Domain and limitations.** L, w, t and rho must be strictly positive; I may be zero. The input-product constraint excludes zero cross-section. Both rectangular shape and uniform current density are assumptions. Use finished rather than nominal copper; etching, neck-downs, contacts, vias, return conductors and temperature-dependent rho require separate consideration. No skin-effect estimate is included. TI-R's 25 °C table gives bulk-copper rho = 1.70 µohm·cm = 1.70e-8 ohm·m; the default does not imply every copper process has this resistivity.

**Independent worked example.** A 60 mm long, 2 mm wide trace with 70 µm finished copper, rho = 1.7e-8 ohm·m and I = 4 A:

1. A = (0.002)(70e-6) = 1.40e-7 m².
2. R = (1.7e-8)(0.060)/(1.40e-7) = 0.007285714285714286 ohm = 7.285714286 milliohm.
3. V_drop = 4R = 0.029142857142857144 V = 29.14285714 mV.

This is the single outgoing conductor, not the loop resistance.

## 2. Legacy IPC-2221 current estimate — expert

**ID:** `pcb.ipc2221_current_estimate`

**References:** [TI-I](https://www.ti.com/lit/ug/tiducs7/tiducs7.pdf), section 3.1.1, Equation 7; [IPC-S public outline](https://www.ipc.org/TOC/IPC-2152.pdf).

TI reproduces the relation DeltaT = `[I/(k A^0.725)]^(1/0.44)`. Solving this one model for current gives I = k DeltaT^0.44 A^0.725. The catalog counts the physical current-screening problem once and displays both layer estimates, not two inversions:

- `area_mil2 = width_mil*thickness_mil`;
- `external_current = 0.048*temperature_rise^0.44*area_mil2^0.725`;
- `internal_current = 0.024*temperature_rise^0.44*area_mil2^0.725`.

Width and thickness are **mil**, not metres or millimetres. One mil = 0.001 inch = 25.4 µm. Area is mil², conductor temperature rise is °C, and the fitted coefficients yield amperes. The coefficients carry the necessary empirical unit dependence; this is not a dimensionally universal SI power law.

**Domain and limitations.** Positive width/thickness are required. The catalog imposes a clearly labeled screening policy of 1–100 °C selected rise; it is **not** asserted to be an experimentally certified validity interval. No source-backed universal geometric applicability range was found in the readable TI reproduction; positive values alone do not prove the curve fit applies to arbitrary geometry. The user must not extrapolate it into a guaranteed thermal or safety rating. It omits board thickness, copper planes, laminate conductivity, airflow, ambient conditions, nearby heat sources, neck-downs and transient/fault duration. Neither the numerical output nor the usual internal/external factor of two is guaranteed conservative. This is not an IPC-2152 implementation and does not establish IPC compliance. Board-specific thermal analysis/prototype measurements and electrical-drop checks remain necessary.

**Independent worked example.** w = 40 mil, t = 1.4 mil and selected rise = 20 °C:

1. A = 40 × 1.4 = 56 mil².
2. ln(I_external) = ln(0.048) + 0.44 ln(20) + 0.725 ln(56).
3. Exponentiating gives I_external = 3.3199439759510987 A.
4. I_internal uses half the coefficient and is 1.6599719879755493 A.

The expected values were calculated with independent high-precision decimal arithmetic, not a backend expression evaluator. Expertise here is recognizing the missing thermal physics and the model's non-certifying status, not the arithmetic complexity.

## 3. Plated via-barrel resistance — expert

**ID:** `pcb.via_barrel_resistance`

**Reference:** [TI-R](https://www.ti.com/lit/pdf/slup224), Topic 4, section II, Table 1 and Figure 3.

Let d be the **finished inside diameter**, p the radial copper wall thickness and L the traversed axial barrel length. The outside copper diameter is d + 2p. Subtracting the inside disk from the outside disk gives the axial metal area:

A = `(pi/4)[(d+2p)^2-d^2] = pi p(d+p)`.

This retains the p² term that is dropped by the thin-wall approximation pi dp. Hence:

- `barrel_copper_area = pi*plating_thickness*(finished_diameter+plating_thickness)`, m²;
- `single_resistance = plated_resistivity*barrel_length/barrel_copper_area`, ohms;
- `bank_resistance = single_resistance/via_count`, ohms.

L, d and p are metres, rho_plated is ohm·m, and N is an integer count. Units match the straight-conductor resistance derivation, but the barrel's physical geometry and process resistivity are different.

**Domain and limitations.** Positive L, d, p and rho are required; N >= 1 and `N == floor(N)` reject fractional via counts. The cross-section constraint guards the denominator. The traversed length may be less than board thickness for a connection between internal layers. Holes must be circular and plating uniform; no copper filling, pad spreading, contact resistance or barrel defects are included. Equal parallel sharing requires symmetric connections and negligible unequal trace resistance. A resistance estimate does **not** specify current capacity.

TI's historical illustrative plated-copper value is 6.0 µohm·cm = 6e-8 ohm·m, substantially above its bulk-copper table value. This is exposed as an editable input, not presented as a universal plating property. Modern supplier/measured values should replace it when available.

**Independent worked example.** L = 1.6 mm, d = 0.30 mm, p = 25 µm, rho_plated = 6e-8 ohm·m, N = 4:

1. Outside diameter = 0.30 + 2(0.025) = 0.350 mm.
2. A = `(pi/4)(0.350² - 0.300²)` = 0.02552544031041707 mm² = 2.552544031041707e-8 m².
3. R_single = (6e-8)(0.0016)/A = 0.003760953732140788 ohm.
4. R_bank = R_single/4 = 0.0009402384330351971 ohm.

The example uses the disk-difference geometry rather than evaluating the catalog's area expression, providing an independent arithmetic path.

## 4. Nominal copper weight to thickness — basic

**ID:** `pcb.nominal_copper_thickness`

**References:** [JLC-C](https://jlcpcb.com/help/article/jlcpcb-copper-weight), explicit copper-weight definition; [NIST-U](https://www.nist.gov/document/2026-hb-133-appendix-e), international-inch conversion.

PCB copper weight is a manufacturing designation in oz/ft², historically referring to copper distributed over an area. JLCPCB explicitly maps 1 oz copper to **nominal** 35 µm. The model deliberately implements that declared manufacturing convention:

- `nominal_thickness = copper_weight*35e-6`, metres;
- `nominal_thickness_mil = nominal_thickness/25.4e-6`, mil.

The first factor is a nominal convention; the second is an exact length conversion. Fractional positive weight designations are allowed, but availability is supplier-specific.

**Domain and limitations.** Copper weight must be positive. Ounces here are neither fluid ounces nor troy ounces. This calculator is **not** the exact physical conversion `mass/(density*area)`: the familiar rounded 35 µm manufacturing convention must not be relabeled an exact SI mass-density equality. It does not determine the thickness at a specific point after etching and plating, or automatically distinguish base foil from finished outer-layer copper. Feed fabricator-guaranteed/measured finished thickness into resistance/current calculations rather than blindly forwarding this nominal result.

**Independent worked example.** A 2 oz/ft² designation:

1. Nominal thickness = 2 × 35 µm = 70 µm = 0.000070 m.
2. Nominal mil = 70/25.4 = 2.7559055118110236 mil.

No density assumption or exact areal-mass claim is introduced.

## 5. Differential routed-length skew — intermediate

**ID:** `pcb.differential_length_skew`

**References:** [TI-S](https://www.ti.com/lit/pdf/snaa103), Chapter 6, Minimizing Skew; [TI-D](https://www.ti.com/lit/an/spracp4a/spracp4a.pdf), section 2.3.

For the same propagation-delay density tau_l on both legs, their common delay component cancels in subtraction. The residual intra-pair skew is:

- `signed_length_mismatch = positive_length-negative_length`, m;
- `signed_skew = signed_length_mismatch*delay_per_length`, s;
- `skew_magnitude = abs(signed_skew)`, s.

The conversion has units `m*(s/m) = s`. Positive signed skew means the positive trace is longer and arrives later. Providing tau_l as an input avoids inventing a dielectric/impedance solver in this lane. TI's 6.84 ps/mm illustrative scale converts to 6.84e-9 s/m and is the editable default.

**Domain and limitations.** Both lengths can be zero or positive; tau_l must be strictly positive. Equal lengths give exact zero skew. Legs must share the same stackup, material/velocity environment and definition of routed length. Package delay, connector/via asymmetry, layer changes, glass weave, dispersion, velocity mismatch and coupled serpentine shortcuts are excluded. A pair routed in different environments can have nonzero skew even with equal lengths. No universal interface tolerance is imposed, and no absolute transmission-line delay or impedance is computed.

**Independent worked example.** L_positive = 80 mm, L_negative = 77.5 mm and tau_l = 6.84 ps/mm:

1. Mismatch = 80 - 77.5 = 2.5 mm = 0.0025 m.
2. Signed skew = 2.5 × 6.84 = 17.10 ps = 1.710e-11 s.
3. Magnitude = 1.710e-11 s.

This example uses millimetres/picoseconds before conversion to SI, independently of the catalog expression units.

## 6. Annular ring with registration allowance — expert

**ID:** `pcb.annular_ring_registration`

**Reference:** [EC-A](https://www.eurocircuits.com/iar-inner-annular-ring/), production tool-size formula and off-centre hole discussion.

For concentric circles the copper ring width is the radial difference between pad and production hole. With worst-case center displacement e, the narrowest side loses exactly e while the opposite side gains e:

- `nominal_ring = (pad_diameter-tool_diameter)/2`, m;
- `minimum_ring = nominal_ring-registration_offset`, m.

The subtraction is ordinary Euclidean circle geometry, not an empirical rule. The source's example uses tool size = finished PTH diameter + 0.10 mm. That particular allowance is **not** hard-coded: the user's fabricator must supply the appropriate production size. Finished-hole diameter alone would overestimate the inner annular ring.

**Domain and limitations.** Both diameters are positive, pad diameter exceeds tool diameter, and radial registration offset is nonnegative. The intact-ring geometry is enforced by `2*registration_offset <= pad_diameter-tool_diameter`. Equality gives tangency/zero ring and is a geometric result, not an acceptable manufacturing pass. Breakout is rejected with an explicit message. Registration offset is radial distance, not a diameter tolerance or independent X/Y bound; combine X/Y bounds into a radial worst case before supplying it. Use the smallest relevant pad and largest relevant production hole, including their tolerances. Noncircular pads, etch variations around the circumference and layer-specific acceptance classes are not modeled. The calculator never assigns an IPC class or supplier compliance status.

**Independent worked example.** Worst-case pad = 0.800 mm, production tool = 0.400 mm and radial offset = 0.050 mm:

1. Pad radius = 0.400 mm; hole radius = 0.200 mm.
2. Concentric ring = 0.400 - 0.200 = 0.200 mm = 0.000200 m.
3. Narrowest residual ring = 0.200 - 0.050 = 0.150 mm = 0.000150 m.

The complete-hole condition holds because offset 0.050 mm is smaller than the concentric ring 0.200 mm. Expert use requires interpreting manufacturing tolerances and drill allowance rather than merely inserting a finished drill number.

## 7. Local trace Joule heat and pulse energy — intermediate

**ID:** `pcb.trace_joule_heat_density`

**References:** [MTU-J](https://multiphysics.geo.mtu.edu/thermoelectromagnetics.html), Joule Heating; [TI-R](https://www.ti.com/lit/pdf/slup224), copper resistivity and geometry.

The university source derives the local electrical-to-thermal conversion density as J·E. For isotropic ohmic copper, E = rho J; therefore q_v = rho J². For a uniform trace A = wt, current density J = I/A. Integrating q_v over volume Lwt and a constant-current pulse duration tau gives:

- `current_density = pulse_current/(width*thickness)`, A/m²;
- `volumetric_heat_rate = resistivity*current_density^2`, W/m³;
- `generated_power = volumetric_heat_rate*length*width*thickness`, W;
- `deposited_energy = generated_power*duration`, J.

Unit check: `(ohm m)*(A/m²)² = W/m³`. Integration over volume gives W, and over time gives J. Although the total power reduces algebraically to I²R, the physical output is the **local volumetric heat-generation source**, followed by its spatial/time integral, not another generic resistor-power task.

**Domain and limitations.** Positive L, w, t and rho; nonnegative constant pulse current and pulse duration. The cross-section guard protects division. Zero current yields zero density/power/energy; zero duration yields zero integrated energy. Constant rho and uniform current are assumptions. Strong temperature-dependent resistivity, skin/proximity effect, neck-downs and contact/via losses are excluded. The energy is **generated** in the conductor, not necessarily retained there. No heat capacity, cooling model, temperature rise, melting, fusing or steady-current rating is claimed. This stays outside the thermal lane's temperature-response models.

**Independent worked example.** L = 50 mm, w = 1 mm, t = 35 µm, rho = 1.7e-8 ohm·m, I = 3 A for 0.2 s:

1. A = 1 mm × 0.035 mm = 0.035 mm² = 3.5e-8 m².
2. J = 3/0.035 = 85.7142857143 A/mm² = 85,714,285.71428572 A/m².
3. q_v = (1.7e-8)J² = 124,897,959.18367347 W/m³.
4. V_copper = 50 × 1 × 0.035 = 1.75 mm³ = 1.75e-9 m³.
5. P = q_v V_copper = 0.21857142857142858 W.
6. E = P(0.2) = 0.04371428571428571 J.

As an independent integrated check, R = (1.7e-8)(0.05)/(3.5e-8) = 0.0242857142857 ohm, and 3²R gives the same total power. This does not provide a temperature answer.

## 8. Rectangular stencil aperture area ratio — intermediate

**ID:** `pcb.stencil_aperture_area_ratio`

**Reference:** [TI-A](https://www.ti.com/lit/an/spraav2a/spraav2a.pdf), section 6.1 Area Ratio.

TI defines area ratio as aperture opening area divided by the interior wall area wetted by paste. Its cited numerical 0.66 guideline is for round fine-pitch BGA apertures. The same geometric definition applies to a rectangular vertical-wall opening with length L, width W and stencil thickness T:

- `opening_area = aperture_length*aperture_width`, m²;
- `wall_area = 2*(aperture_length+aperture_width)*stencil_thickness`, m²;
- `area_ratio = opening_area/wall_area`, dimensionless;
- `ratio_margin = area_ratio-target_area_ratio`, dimensionless.

The wall area is the sum of four wall rectangles: LT + LT + WT + WT. This is an independent geometric derivation rather than copying an unrelated circular expression. For a square opening it simplifies to L/(4T), consistent with the circular-diameter form D/(4T), but neither simplification is counted as another calculator.

**Domain and limitations.** L, W, T and selected target must be strictly positive; a positive-wall-area constraint guards the division. Negative margin is a valid below-target result, not an invalid geometric input. The default target 0.66 is an explicitly labeled **screening extrapolation** of TI's round-aperture guideline; the assembler must approve any use for rectangular apertures. Actual transfer depends on paste particle size/rheology, wall coating/finish, aperture aspect ratio, rounding/taper, stencil spacing and printing settings. A straight rectangular opening is assumed; stepped or tapered stencils need the actual wetted areas. The output is neither solder volume nor measured transfer efficiency or certification.

**Independent worked example.** L = 0.50 mm, W = 0.30 mm, T = 0.10 mm, target = 0.66:

1. Opening area = 0.50 × 0.30 = 0.150 mm² = 1.5e-7 m².
2. Wall area = 2(0.50 + 0.30)(0.10) = 0.160 mm² = 1.6e-7 m².
3. AR = 0.150/0.160 = 0.9375.
4. Signed margin = 0.9375 - 0.66 = 0.2775.

The positive geometric margin alone does not establish reliable release.

## Implementation and handoff

All formulas use only the contract's permitted arithmetic, powers, `abs`, `floor`, and `pi`. No branching, string-valued layer selection, interpolation, arrays, implicit units, network fetching, or hidden lookup table is required. Both IPC layer estimates are outputs of one calculation. The via count uses an input-only integer constraint. Annular-ring constraints are expanded directly in terms of input diameters/offset because constraints run before outputs. Every supporting output has an independently computed numerical expected value.

Defaults are finite, positive where divisions/empirical powers require positivity, and satisfy the declared intact-ring and integer-count domains by construction. Worked expected numbers were obtained with separate high-precision decimal arithmetic and the derivations above, not copied from an application engine. The relative tolerance in each example is 1e-6; it accommodates numerical floating-point rounding and is **not** an uncertainty bound on the physical model.

No builds, tests, linters, formatters or engine smoke runs were performed by this research lane. Integration should exercise each example and every domain constraint, equal-length zero skew, zero-current/zero-duration heating, annular tangency versus rejected breakout, fractional-via rejection, and below-target stencil margin. Assumptions and source caveats should remain visible in the desktop UI rather than hiding them behind a numerical current or pass/fail label.
