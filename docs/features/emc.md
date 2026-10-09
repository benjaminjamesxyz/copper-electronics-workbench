# EMC design calculators

Rust evaluates the catalog; Flutter supplies static explanatory diagrams only. Shared registration and independent numerical/native acceptance belong to integration. No implementation checks were run by this lane.

Research scope and evidence

This lane promotes the six researched definitions into `backend/catalog/emc.json`, with exact-ID diagrams in `frontend/lib/diagrams/emc.dart`. Level distribution: one basic, three intermediate, two expert. All quantities use SI unless explicitly dimensionless or dB. Defaults equal the complete worked examples below.

Read `coordination/EXPANSION_CONTRACT.md`, `coordination/CONTRACT.md`, the board's expansion allocation, all 128 baseline formulas and the expansion communication bus before selection. RF explicitly confirmed that unintended short-wire radiation remains EMC-owned. Protection excludes snubbers; cables excludes shielding; logic confirmed no spectral/dither overlap. Uniform-field loop pickup was considered and rejected because it merely inverts the existing sinusoidal magnetic-flux model.

Search boundary: English-language manufacturer and university sources accessible on 2026-10-09; no date restriction on foundational equations. Searches covered aperture shielding, coupled input-filter mismatch, switching-waveform Fourier spectra, sinusoidal frequency spreading, measured ringing/snubber design, and elementary radiated fields. Excluded generic dB conversion, skin depth, displacement current, baseline capacitive crosstalk, intentional antennas/links, component coil design and detector compliance simulation. This is a targeted engineering source review, not an exhaustive systematic literature review.

Sources read during the research phase (not independently re-fetched during implementation):

1. **Laird technical note 154, Gary Fenical, “Rule-of-Thumb for Calculating Aperture Size,” page 1.** Manufacturer engineering note. [Primary hosted PDF](http://cdn.lairdtech.com/home/brandworld/files/EMI%20Rule-of-Thumb%20for%20Calculating%20Aperture%20Size%20Technical%20Note%20Download.pdf). Supports the isolated thin-slot equation and small-aperture guidance. Moderate confidence as a screening approximation, not a bound for arbitrary enclosures. HTTPS failed with an expired certificate; the HTTP primary-host copy was successfully read. The equation, not the loosely labeled page-two chart, is authoritative here.
2. **Texas Instruments, Art Kay, “Common mode and differential filter,” 2022-08-23, pages 2–5.** Manufacturer circuit/simulation teaching note. [Primary PDF](https://e2e.ti.com/cfs-file/__key/communityserver-discussions-components-files/73/common-mode-and-differential-filter.pdf). Shows common-mode conversion from unequal shunt capacitors and attenuation by the bridging differential capacitor. The exact general R1/R2 nodal formula below is independently derived from that topology; it is not claimed as a verbatim source equation.
3. **Texas Instruments, John Rice, Dirk Gehrke and Mike Segal, “Understanding Noise-Spreading Techniques and their Effects in Switch-Mode Power Applications,” section IV equation (1), Figure 2; section IX, Figure 7; sections V–VI.** Manufacturer seminar. [Primary PDF](https://www.ti.com/download/trng/docs/seminar/Topic_2_Rice_Gehrke_Segal.pdf). Supports the trapezoidal harmonic and Carson harmonic-occupancy models, plus the important distinction between line amplitudes and EMI receiver readings. High confidence in ideal Fourier identity; moderate confidence in Carson occupied-band approximation. The two models represent different physical mechanisms: finite edge shape versus frequency modulation.
4. **Texas Instruments SLVA790A, Helen Chen, “Reducing Radiated EMI in TPS61088 Boost Converter,” section 2.4, equations (4)–(5).** Manufacturer application report, revised 2019. [Primary PDF](https://www.ti.com/lit/pdf/slva790). Supports capacitor-added ringing identification, snubber damping guideline, capacitance ratio and loss. Section 2.5 provides context for unintended cable radiation, but its chamber-specific coefficient is deliberately not used in the free-space model.
5. **MIT 6.013, “Electromagnetics and Applications,” Chapter 10, section 10.2.1, equations (10.2.7)–(10.2.10).** University derivation. [Primary course PDF](https://ocw.mit.edu/courses/6-013-electromagnetics-and-applications-spring-2009/1b0b8fccbc37c9089f313dc459bf0b14_MIT6_013S09_chap10.pdf). Supports Hertzian-current far-zone radiation. High confidence in the ideal field equation; practical cable current-distribution uncertainty remains potentially dominant.

ADI AN-671 and its web article timed out and were not used as evidence. The TI filter blog resolved to an uninformative RSS feed; the accessible Art Kay PDF replaced it. Search-result summaries alone were not accepted as evidence. No measured hardware, enclosure, instrument or compliance evidence was collected.

**Universal limit:** every result is an engineering estimate, not EMC certification, a safety approval, or a substitute for actual conducted/radiated/immunity testing. Manufacturer notes can establish useful starting models but cannot certify a different assembly.


## 1. `emc.thin_slot_shielding` — basic

### Purpose and novelty

Estimate whether a single thin shield slot is likely to dominate electric-field leakage. Closest baseline: `magnetics.conductor_skin_depth`, which predicts penetration into bulk conductive material. This model predicts leakage through an opening using aperture geometry, not bulk absorption, skin depth or an inverse of that calculation. RF's circular-aperture antenna gain concerns intentional radiation and is not this shield-slot model.

### Model, units and limits

With frequency f [Hz], slot length L [m], width w [m], shield thickness t [m], and c=299792458 m/s:

- wavelength = c/f [m]
- shielding_db = 20 log10(wavelength/(2L)) [dB]
- field_ratio = 2L/wavelength [dimensionless field-amplitude ratio]

Positive SE means attenuation, not gain. The source describes a worst-orientation isolated slot with L>w and L much greater than t. The catalog operationalizes “much greater” as L>=10t, an explicit screening convention, not a standard limit. L<=wavelength/20 follows the source's recommended upper small-aperture dimension. t=0 is the ideal zero-thickness limit. Width and thickness validate geometry but do not enter the thin-slot leading-order equation.

Do not apply this to round apertures, aperture arrays, cavity resonance, significant wall-depth waveguide attenuation, shielding seams or low-frequency magnetic near fields. It cannot establish the shielding of an entire enclosure.

### Complete independent example

Inputs: frequency=100000000 Hz; slot_length=0.02 m; slot_width=0.002 m; shield_thickness=0.0005 m.

- wavelength = 299792458/100000000 = **2.99792458 m**.
- wavelength/(2L) = 74.9481145.
- shielding_db = 20 log10(74.9481145) = **37.495214231999306 dB**.
- field_ratio = 0.04/2.99792458 = **0.013342563807926084**.

Geometry checks: L/w=10, L/t=40, L/wavelength=0.006671281903963042.

### Independent tester cases

- Nominal: every output above, relative tolerance 1e-6.
- Small-aperture endpoint: frequency=100000000, L=0.149896229, w=0.002, t=0.0005 gives wavelength=2.99792458, SE=20 dB and field_ratio=0.1; accept equality within arithmetic precision.
- Halve nominal L while retaining valid w/t: SE increases by 6.020599913279624 dB and field_ratio halves.
- t=0 is valid and changes no result; t=L/10 is valid.
- Reject f<=0, L<=0, w<=0, t<0, w>=L, t>L/10 and L>wavelength/20. No compliance label should appear for valid results.

### Mandatory diagram

700×280 canvas, 16 px margins, labels at least 14 px. Draw a shield front face at x=170..430, y=65..200 with one visibly long horizontal slot; show a separate thin edge inset at x=510..620 for wall thickness. Incoming and transmitted field arrows cross the slot, not the solid metal. Dimension the longest slot edge as `slot_length`, its narrow opening as `slot_width`, and inset thickness as `shield_thickness`. Bind a frequency label to `frequency`, the outgoing/incoming amplitude ratio to `field_ratio`, and an external result label to `shielding_db`. `wavelength` may appear in a separate legend. Do not portray width as equal to thickness. Caption: “One thin, electrically small slot; worst-orientation field-leakage estimate. Enclosure resonances and other openings are not modeled.” No simulated field map.


## 2. `emc.rc_common_mode_conversion` — expert

### Purpose and novelty

Predict differential noise created by mismatched input EMI-filter legs and suppress it with a bridging capacitor. Closest baselines: `filters.rc_lowpass_response` describes one scalar branch, and `opamps.difference_ratio_error` describes a resistive active amplifier. Neither predicts common-mode conversion through a coupled three-capacitor input network. This is also not the baseline injected capacitive-crosstalk pulse model.

### Topology and exact derivation

Both ideal source terminals have the same sinusoidal voltage Vcm relative to a common reference. R1 feeds node 1 and R2 feeds node 2. C1 connects node 1 to the reference; C2 connects node 2 to the reference. Cd connects node 1 directly to node 2. The receiver has infinite input impedance. Vdiff=V1−V2.

With s=jω, nodal equations after multiplying each by its branch resistance are:

- [1+sR1(C1+Cd)] V1 − sR1 Cd V2 = Vcm
- −sR2 Cd V1 + [1+sR2(C2+Cd)] V2 = Vcm

The determinant is 1+sA+s²B, where A=R1(C1+Cd)+R2(C2+Cd) [s] and B=R1R2[C1C2+Cd(C1+C2)] [s²]. Subtracting the two numerator solutions cancels both Cd terms:

- mismatch_time = D = R2C2−R1C1 [s]
- Vdiff/Vcm = sD/(1+sA+s²B)
- conversion_gain = ω|D|/hypot(1−ω²B,ωA)
- differential_rms = common_mode_rms × conversion_gain [V]

The signed D carries branch polarity; the last two outputs are magnitudes. R1/R2 and C1/C2 are positive, Cd and frequency may be zero. Positive passive elements make the denominator nonzero for all admitted finite physical frequencies. At DC conversion is exactly zero. Source resistance belongs in R1/R2; finite ADC loading and nonlinear RF rectification do not.

Evidence is the TI topology and mismatch mechanism; the general algebra is our independently solved ideal network. No claim is made that a large Cd makes the complete receiver immune.

### Complete independent example

Inputs: resistance_one=resistance_two=1000 ohm; capacitance_one=1.01e-8 F; capacitance_two=9.9e-9 F; differential_capacitance=1e-7 F; frequency=10000 Hz; common_mode_rms=0.1 V.

- R1C1=10.1 microseconds; R2C2=9.9 microseconds.
- mismatch_time = **−2e-7 s**.
- A=0.00022 s; B=2.09999e-9 s².
- ω=62831.85307179586 rad/s, 1−ω²B=−7.290428218497455, ωA=13.82300767579509.
- Solving the two complex nodal equations independently gives |V1/Vcm−V2/Vcm| = **0.0008041073247070083** = conversion_gain.
- differential_rms = 0.1×conversion_gain = **0.00008041073247070084 V**.

Complex two-by-two inversion, not the Rust expression evaluator, generated the gain oracle.

### Independent tester cases

- Nominal: every output above at 1e-6 relative tolerance.
- Set capacitance_one=capacitance_two=1e-8: D, gain and differential voltage are exactly zero, regardless of Cd.
- Set frequency=0: retain D=−2e-7 s, but both amplitude outputs are zero.
- Set Cd=0: D remains −2e-7 s, gain=**0.009009434213906478**, differential_rms=**0.0009009434213906478 V**. Independently subtract two uncoupled complex RC responses.
- Swap complete branch pairs: D changes sign; both magnitudes are unchanged.
- Set common_mode_rms=0: gain remains nominal and voltage becomes zero.
- Reject nonpositive R/C1/C2, negative Cd/frequency/common-mode RMS and nonfinite values. Huge arithmetic intermediates must fail explicitly, not return plausible finite zero through a nonfinite normalization.

### Mandatory diagram

Place common source bus x=70, node-one rail y=85, node-two rail y=185. Each rail passes through its own resistor, with wires broken at resistor bodies, to receiver terminals x=570. Connect C1 from node one down to a clearly separate common-return rail; connect C2 from node two to that same return without accidentally shorting the signal rails. Place Cd vertically between node-one and node-two junctions near x=430. Use explicit junction dots for each branch. Route the shunt capacitor returns outside the bridge region, with no ambiguous crossings. Bind `resistance_one`, `resistance_two`, `capacitance_one`, `capacitance_two`, `differential_capacitance`, and `common_mode_rms`. Mark output + at node one and − at node two; bind its magnitude to `differential_rms`. Caption: “Equal source interference becomes differential through RC mismatch; Cd bridges the receiver inputs. Ideal high-impedance receiver.” Do not draw a differential capacitor to ground.

