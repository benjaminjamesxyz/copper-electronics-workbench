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

