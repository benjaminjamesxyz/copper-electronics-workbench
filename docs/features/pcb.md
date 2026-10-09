# PCB interconnect and manufacturing calculators

These offline, closed-form calculators execute through the shared Rust engine using [the PCB catalog](../../backend/catalog/pcb.json). They do not run a simulator or certify a board. The complete lane comprises two basic, three intermediate and three expert physical models, as researched in [PCB derivations](../../research/pcb.md). Outputs are evaluated in order; constraints run first and reference only inputs. All inputs must be finite and exactly match the selected model’s required keys. Nonfinite intermediate/final arithmetic is an explicit error, not a usable result.

Every worked example below includes all expected outputs at relative tolerance **1e-6**. That tolerance describes numerical comparison, not physical accuracy, manufacturing yield or safety margin. Source-access claims in the evidence register describe the original research, not a fresh source fetch by the implementation author.

## Source evidence and caveats


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


## pcb.trace_resistance_drop — PCB trace DC resistance and voltage drop

**Level:** basic

**Purpose:** Estimate the DC resistance and voltage-drop magnitude of one uniform rectangular copper trace from its finished geometry.

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

### Inputs, defaults and numerical domain

| Key | Quantity | Unit | Default | Allowed scalar range |
|---|---|---|---|---|
| `length` | Finished trace length | m | 0.1 | > 0.0 |
| `width` | Finished trace width | m | 0.001 | > 0.0 |
| `thickness` | Finished copper thickness | m | 3.5e-05 | > 0.0 |
| `resistivity` | Copper resistivity at operating temperature | ohm m | 1.7e-08 | > 0.0 |
| `current` | DC current magnitude | A | 1.0 | >= 0.0 |

**Input-only guards and error rationale:**

- `width*thickness > 0`: The finished trace must have a positive copper cross-section.

Defaults satisfy these scalar and model conditions by construction. See the derivation’s domain discussion for the physical reason behind each restriction; qualitative assumptions are not inferred from numeric inputs.

### Complete worked vector

| Input key | Example value | Unit |
|---|---|---|
| `length` | 0.06 | m |
| `width` | 0.002 | m |
| `thickness` | 7e-05 | m |
| `resistivity` | 1.7e-08 | ohm m |
| `current` | 4.0 | A |

| Output key (evaluation order) | Independently worked expected value | Unit |
|---|---|---|
| `cross_section` | 1.4e-07 | m^2 |
| `resistance` | 0.007285714285714286 | ohm |
| `voltage_drop` | 0.029142857142857144 | V |


## pcb.ipc2221_current_estimate — Legacy IPC-2221 trace current screening estimate

**Level:** expert

**Purpose:** Compute both legacy external- and internal-layer current estimates for a specified trace cross-section and selected temperature rise; this is not a certified current rating.

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

### Inputs, defaults and numerical domain

| Key | Quantity | Unit | Default | Allowed scalar range |
|---|---|---|---|---|
| `width_mil` | Finished trace width | mil | 20.0 | > 0.0 |
| `thickness_mil` | Finished copper thickness | mil | 1.4 | > 0.0 |
| `temperature_rise` | Selected conductor temperature rise | degC | 10.0 | >= 1.0, <= 100.0 |

**Input-only guards and error rationale:**

- `width_mil*thickness_mil > 0`: Legacy empirical powers require a positive cross-sectional area.

Defaults satisfy these scalar and model conditions by construction. See the derivation’s domain discussion for the physical reason behind each restriction; qualitative assumptions are not inferred from numeric inputs.

### Complete worked vector

| Input key | Example value | Unit |
|---|---|---|
| `width_mil` | 40.0 | mil |
| `thickness_mil` | 1.4 | mil |
| `temperature_rise` | 20.0 | degC |

| Output key (evaluation order) | Independently worked expected value | Unit |
|---|---|---|
| `area_mil2` | 56.0 | mil^2 |
| `external_current` | 3.319943975951099 | A |
| `internal_current` | 1.6599719879755492 | A |


## pcb.via_barrel_resistance — Plated via-barrel DC resistance

**Level:** expert

**Purpose:** Estimate the axial DC resistance of a cylindrical plated through-hole and an ideal equal-current bank of identical vias.

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

### Inputs, defaults and numerical domain

| Key | Quantity | Unit | Default | Allowed scalar range |
|---|---|---|---|---|
| `barrel_length` | Electrically traversed barrel length | m | 0.0016 | > 0.0 |
| `finished_diameter` | Finished inside hole diameter | m | 0.0003 | > 0.0 |
| `plating_thickness` | Radial copper plating thickness | m | 2.5e-05 | > 0.0 |
| `plated_resistivity` | Plated copper resistivity | ohm m | 6e-08 | > 0.0 |
| `via_count` | Identical parallel via count | 1 | 1.0 | >= 1.0 |

**Input-only guards and error rationale:**

- `via_count == floor(via_count)`: The parallel via count must be a positive integer.
- `plating_thickness*(finished_diameter+plating_thickness) > 0`: The plated barrel must have a positive copper cross-section.

Defaults satisfy these scalar and model conditions by construction. See the derivation’s domain discussion for the physical reason behind each restriction; qualitative assumptions are not inferred from numeric inputs.

### Complete worked vector

| Input key | Example value | Unit |
|---|---|---|
| `barrel_length` | 0.0016 | m |
| `finished_diameter` | 0.0003 | m |
| `plating_thickness` | 2.5e-05 | m |
| `plated_resistivity` | 6e-08 | ohm m |
| `via_count` | 4.0 | 1 |

| Output key (evaluation order) | Independently worked expected value | Unit |
|---|---|---|
| `barrel_copper_area` | 2.552544031041707e-08 | m^2 |
| `single_resistance` | 0.003760953732140788 | ohm |
| `bank_resistance` | 0.0009402384330351971 | ohm |


## pcb.annular_ring_registration — Annular ring with hole-registration allowance

**Level:** expert

**Purpose:** Compute the nominal and narrowest residual inner annular ring for a circular copper pad, production drill diameter and radial hole-center displacement.

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

### Inputs, defaults and numerical domain

| Key | Quantity | Unit | Default | Allowed scalar range |
|---|---|---|---|---|
| `pad_diameter` | Worst-case copper pad diameter | m | 0.0006 | > 0.0 |
| `tool_diameter` | Worst-case production drill diameter | m | 0.00035 | > 0.0 |
| `registration_offset` | Maximum radial center displacement | m | 5e-05 | >= 0.0 |

**Input-only guards and error rationale:**

- `pad_diameter > tool_diameter`: Copper pad diameter must exceed production drill diameter.
- `2*registration_offset <= pad_diameter-tool_diameter`: The specified offset causes hole breakout; intact-ring geometry requires offset no larger than the nominal ring.

Defaults satisfy these scalar and model conditions by construction. See the derivation’s domain discussion for the physical reason behind each restriction; qualitative assumptions are not inferred from numeric inputs.

### Complete worked vector

| Input key | Example value | Unit |
|---|---|---|
| `pad_diameter` | 0.0008 | m |
| `tool_diameter` | 0.0004 | m |
| `registration_offset` | 5e-05 | m |

| Output key (evaluation order) | Independently worked expected value | Unit |
|---|---|---|
| `nominal_ring` | 0.0002 | m |
| `minimum_ring` | 0.00015 | m |


## pcb.nominal_copper_thickness — Nominal PCB copper weight to thickness

**Level:** basic

**Purpose:** Convert a PCB manufacturing copper-weight designation to nominal thickness using the explicit 35 micrometres per ounce convention.

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

### Inputs, defaults and numerical domain

| Key | Quantity | Unit | Default | Allowed scalar range |
|---|---|---|---|---|
| `copper_weight` | Nominal copper-weight designation | oz/ft^2 | 1.0 | > 0.0 |

**Input-only guards and error rationale:**

- No additional relational constraints; the scalar ranges enforce this model’s domain.

Defaults satisfy these scalar and model conditions by construction. See the derivation’s domain discussion for the physical reason behind each restriction; qualitative assumptions are not inferred from numeric inputs.

### Complete worked vector

| Input key | Example value | Unit |
|---|---|---|
| `copper_weight` | 2.0 | oz/ft^2 |

| Output key (evaluation order) | Independently worked expected value | Unit |
|---|---|---|
| `nominal_thickness` | 7e-05 | m |
| `nominal_thickness_mil` | 2.7559055118110236 | mil |


## pcb.differential_length_skew — Differential-pair routed length-mismatch skew

**Level:** intermediate

**Purpose:** Calculate signed and magnitude intra-pair timing skew from the two routed lengths and a supplied stackup propagation-delay density.

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

### Inputs, defaults and numerical domain

| Key | Quantity | Unit | Default | Allowed scalar range |
|---|---|---|---|---|
| `positive_length` | Positive-leg routed length | m | 0.1 | >= 0.0 |
| `negative_length` | Negative-leg routed length | m | 0.099 | >= 0.0 |
| `delay_per_length` | Common stackup propagation-delay density | s/m | 6.84e-09 | > 0.0 |

**Input-only guards and error rationale:**

- No additional relational constraints; the scalar ranges enforce this model’s domain.

Defaults satisfy these scalar and model conditions by construction. See the derivation’s domain discussion for the physical reason behind each restriction; qualitative assumptions are not inferred from numeric inputs.

### Complete worked vector

| Input key | Example value | Unit |
|---|---|---|
| `positive_length` | 0.08 | m |
| `negative_length` | 0.0775 | m |
| `delay_per_length` | 6.84e-09 | s/m |

| Output key (evaluation order) | Independently worked expected value | Unit |
|---|---|---|
| `signed_length_mismatch` | 0.0025 | m |
| `signed_skew` | 1.71e-11 | s |
| `skew_magnitude` | 1.71e-11 | s |

