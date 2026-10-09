# EMC expansion research: six new physical design models

## Scope and evidence

Own six definitions in `research/expansion/emc.json`; no production code, shared registry, tests or existing catalog changes. Level distribution: one basic, three intermediate, two expert. All quantities use SI unless explicitly dimensionless or dB. Defaults equal the complete worked examples below.

Read `coordination/EXPANSION_CONTRACT.md`, `coordination/CONTRACT.md`, the board's expansion allocation, all 128 baseline formulas and the expansion communication bus before selection. RF explicitly confirmed that unintended short-wire radiation remains EMC-owned. Protection excludes snubbers; cables excludes shielding; logic confirmed no spectral/dither overlap. Uniform-field loop pickup was considered and rejected because it merely inverts the existing sinusoidal magnetic-flux model.

Search boundary: English-language manufacturer and university sources accessible on 2026-10-09; no date restriction on foundational equations. Searches covered aperture shielding, coupled input-filter mismatch, switching-waveform Fourier spectra, sinusoidal frequency spreading, measured ringing/snubber design, and elementary radiated fields. Excluded generic dB conversion, skin depth, displacement current, baseline capacitive crosstalk, intentional antennas/links, component coil design and detector compliance simulation. This is a targeted engineering source review, not an exhaustive systematic literature review.

Actually read sources:

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

## 3. `emc.trapezoid_conducted_harmonic` — intermediate

### Purpose and novelty

Predict a selected spectral line from the actual conducted pulse current entering a known resistive RF termination. Closest baselines: `signals.rise_time_bandwidth` has one edge-time bandwidth heuristic, while `power.buck_input_capacitor_rms` integrates total waveform RMS. Neither resolves a selected harmonic with duty-cycle nulls and the second edge-shape sinc factor. This is a Fourier-generation model, not a renamed single-pole filter.

### Model and conventions

Use switching_frequency fs [Hz], pulse_current Ipk [A], duty_half_height D, equal edge_time tr [s], harmonic_order n, and termination_resistance R [ohm]. A pulse rises linearly from zero to Ipk in tr, remains at Ipk, and falls to zero in tr. D/fs is measured between 50%-height crossings. tr is **full 0–100% transition time**, not 10–90% rise time.

This waveform is the convolution of a rectangular pulse of duration D/fs with a unit-area rectangle of duration tr. Hence the two Fourier sinc factors:

- harmonic_frequency = n fs
- dc_current = Ipk D
- harmonic_current_rms = sqrt(2) Ipk |sin(πnD)/(πn)| × |sin(πn fs tr)/(πn fs tr)|
- termination_voltage_rms = R harmonic_current_rms

TI equation (1) gives real sinusoidal peak harmonic amplitude; divide by sqrt(2) for our RMS output. A complex Fourier coefficient alone would differ by a factor of two. n must be a positive integer, 0<D<1, tr>0, and tr fs<=min(D,1−D). Equality permits triangular pulses or zero low plateau. Ideal instantaneous edges are represented by the limit tr→0+, not by dividing by a zero argument. The positive argument constraint catches underflow before evaluation. No arbitrary minimum edge time is imposed.

The current must actually enter R. Do not feed raw switch current into this model and infer a 50-ohm LISN result without the intervening network. Detector bandwidth, quasi-peak weighting, line overlap, jitter and spread spectrum are absent.

### Complete independent example

Inputs: switching_frequency=100000 Hz; pulse_current=0.001 A; duty_half_height=0.4; edge_time=1e-7 s; harmonic_order=3; termination_resistance=50 ohm.

- harmonic_frequency = **300000 Hz**.
- dc_current = 0.001×0.4 = **0.0004 A**.
- Duty angle πnD=1.2π; edge angle πn fs tr=0.03π.
- Peak line amplitude = 2×0.001×0.4×|sin(1.2π)/(1.2π)|×|sin(0.03π)/(0.03π)|.
- harmonic_current_rms = peak/sqrt(2) = **0.00008806826043190418 A**.
- termination_voltage_rms = 50×current = **0.004403413021595208 V**.

### Independent tester cases

- Nominal: all four outputs above at 1e-6 relative tolerance.
- Triangular endpoint: change D=0.5, tr=5e-6 s, n=1, retain fs=100000, Ipk=0.001, R=50. Expected frequency=100000 Hz, DC=0.0005 A, RMS current=2sqrt(2)×0.001/π²=**0.0002865795841253782 A**, RMS voltage=**0.014328979206268907 V**.
- For D=0.5 and n=2, the harmonic is analytically zero; an absolute null tolerance scaled to Ipk times floating-point sine error is appropriate. Never compare a numerical sine residual relatively to mathematical zero.
- Ipk=0 gives zero DC/current/voltage while retaining fn. Complement D with 1−D: harmonic magnitude stays equal, DC changes.
- As tr→0+, the edge factor tends to one. Small representable arguments must not be clamped to an arbitrary floor.
- Reject fractional n, n<1, D endpoints/outside, tr<=0, overlapping edges, fs<=0 and R<=0. A nonfinite/underflowed sine denominator must produce an explicit error.

### Mandatory diagram

Use a static current-versus-time sketch in x=55..420, y=45..195 with zero baseline and one trapezoid. Show 50%-height markers and a width bracket between those markers, labeled “D/fs” with separate keys `duty_half_height` and `switching_frequency`; never bind a duration directly to the dimensionless duty key. Label the full rising-edge bracket `edge_time` and plateau `pulse_current`. At right, show a current arrow into a resistor to return, labeling `termination_resistance` and RMS result `termination_voltage_rms`. Put one conceptual spectral line inset labeled `harmonic_frequency` and `harmonic_current_rms`; do not plot computed line height. Caption: “Illustrative symmetric full-edge trapezoid; the selected Fourier line is RMS. Current is assumed delivered to the RF termination, not inferred from a switch node.”

## 4. `emc.sinusoidal_dither_occupancy` — intermediate

### Purpose and novelty

Select switching-frequency modulation depth and rate while inspecting occupied harmonic bands and their overlap. Closest baseline: `timing.timer_clock_divider` has a static divided clock frequency, and `signals.rise_time_bandwidth` estimates unmodulated edge bandwidth. The new physical mechanism is FM sideband redistribution. It does not duplicate the unmodulated trapezoid line amplitude above or measurement's sampling-jitter SNR.

### Model and limits

Instantaneous frequency is fs+Δf sin(2πfm t), where Δf is a **peak**, not peak-to-peak, frequency deviation. Phase modulation index of the nth harmonic is βn=nΔf/fm. TI section IX applies Carson's rule to switching harmonics:

- harmonic_modulation_index = nΔf/fm
- occupied_bandwidth Bn = 2(nΔf+fm) [Hz]
- next_harmonic_gap = fs−[Bn+B(n+1)]/2 = fs−[(2n+1)Δf+2fm] [Hz]

Bn is centered at n fs; the next band is centered at (n+1)fs. The gap is signed. Negative means the approximate occupied intervals overlap, **not** that the hardware is invalid. Sidebands exist outside Carson intervals, so positive gap does not prove spectral isolation.

The assumed fixed waveform is phase/frequency modulated, and fm<fs provides the slower switching-dither interpretation. Δf<fs keeps instantaneous switching positive. This model assumes neither uniform spectral density nor any dB reduction. Carson bandwidth approximates roughly 98% power for sinusoidal FM, not a strict cutoff or universal measured-bandwidth definition. For β≪1 it intentionally tends toward 2fm even though weak sidebands contain little power; the exact zero-deviation carrier is excluded because this approximation would misleadingly retain modulation bandwidth. Changes to converter control dynamics, audible noise and operating mode remain outside this spectral estimate.

### Complete independent example

Inputs: switching_frequency=100000 Hz; peak_deviation=10000 Hz; modulation_frequency=2000 Hz; harmonic_order=3.

- harmonic_modulation_index = 3×10000/2000 = **15**.
- occupied_bandwidth = 2×(30000+2000) = **64000 Hz**.
- Third band endpoints: 300000±32000 = 268000..332000 Hz.
- Next band width = 2×(40000+2000) = 84000 Hz; lower edge = 400000−42000 = 358000 Hz.
- next_harmonic_gap = 358000−332000 = **26000 Hz**.

### Independent tester cases

- Nominal: all three outputs above.
- n=5 with other defaults: index=25, width=104000 Hz, gap=−14000 Hz. Accept the negative gap.
- Small positive deviation=1 Hz: index=0.0015, width=4006 Hz, gap=95993 Hz. This demonstrates Carson's conservative narrowband behavior, not actual 98%-power measurement.
- At fixed Δf/fm, scale fs, Δf and fm equally: index stays fixed; width and gap scale.
- Reject Δf<=0, Δf>=fs, fm<=0, fm>=fs, fs<=0, noninteger n or n<1. The valid no-overlap/overlap crossing is not a rejection boundary.

### Mandatory diagram

Frequency axis x=65..635 at y=205. Draw two static shaded/outlined bands centered at n fs and (n+1)fs; identify their centers with descriptive text, not incorrectly bound live numbers. Dimension the first band's full width with `occupied_bandwidth`. Dimension the separation between adjacent band edges with `next_harmonic_gap`, noting signed overlap in caption. Add a top-left inset showing sinusoidal instantaneous frequency around fs; label its center `switching_frequency`, peak vertical deviation `peak_deviation`, and modulation rate `modulation_frequency`. Label selected order `harmonic_order` and index `harmonic_modulation_index` in a clear legend. Caption: “Illustrative Carson intervals for sinusoidal dither; negative gap denotes overlap. No detector reduction or hard spectral boundary is predicted.” The static bands are not dynamically fitted to outputs.

## 5. `emc.ringing_rc_snubber` — expert

### Purpose and novelty

Infer physical switch-node parasitics from an added-capacitor experiment, then select a shunt series-RC suppression network and budget charge/discharge loss. Closest baselines: `transients.rlc_underdamped_step` predicts a known series RLC circuit, and `mosfet.gate_loop_damping_resistor` inserts a resistor in a gate loop. This model identifies unknown node parasitics and designs a different topology with a DC-blocking snubber capacitor. It is not another RLC inverse mode: two independent frequency measurements plus a known perturbation identify both L and C, then constrain the practical suppression network.

### Derivation and source qualification

Let f0 be original ringing frequency, f1 the lower frequency with known Ctest shunting the same nodes. For the same lightly damped mode:

f0²=1/(4π² Lp Cp), f1²=1/[4π² Lp(Cp+Ctest)]. Therefore:

- parasitic_capacitance Cp=Ctest/[(f0/f1)²−1]
- parasitic_inductance Lp=1/[(2πf0)² Cp]
- snubber_resistance Rs=sqrt(Lp/Cp)/(2ζ)
- snubber_capacitance Cs=k Cp
- snubber_power P=Cs V² fs

The Cp expression uses ((f0−f1)/f1)((f0+f1)/f1), retaining the small measured difference instead of subtracting two nearly equal squared ratios. Lp is evaluated with factored square roots; power squares V sqrt(Cs) sqrt(fs). SI units: Hz, F, H, ohm, V and W.

TI directly describes the half-frequency experiment, where Ctest=3Cp. The general two-frequency identification is its algebraic extension. ζ is a **first-pass design target** in TI's shunt-snubber approximation; 0.5<=ζ<=1. It is not an asserted exact damping ratio of the full third-order RC network. Use 3<=k<=4, then tune on hardware. f0>f1>fs is enforced; it is necessary but not sufficient to establish a clean separated ringing mode. Measurement uncertainty explodes when f0 and f1 are close; the catalog does not invent a minimum frequency shift. Both switch states must be long enough for complete snubber capacitor charge/discharge through V. P is the ordinary capacitive cycle loss, not a complete measured thermal-loss prediction including ringing energy.

### Complete independent example

Inputs: original_frequency=30000000 Hz; loaded_frequency=15000000 Hz; test_capacitance=3e-9 F; damping_target=0.75; capacitance_multiplier=3; voltage_swing=12 V; switching_frequency=500000 Hz.

- Ratio f0/f1=2, so parasitic_capacitance = 3e-9/(4−1) = **1e-9 F**.
- parasitic_inductance = 1/[(2π×30e6)²×1e-9] = **2.8144773233982716e-8 H**.
- sqrt(Lp/Cp)=5.305164769729845 ohm.
- snubber_resistance = 5.305164769729845/(2×0.75) = **3.53677651315323 ohm**.
- snubber_capacitance = 3×1e-9 = **3e-9 F**.
- snubber_power = 3e-9×12²×500000 = **0.216 W**.

### Independent tester cases

- Nominal: all five outputs above at 1e-6 relative tolerance.
- Set ζ=1 and k=4: Cp and Lp unchanged; Rs=**2.6525823848649224 ohm**, Cs=**4e-9 F**, P=**0.288 W**. Both endpoints are valid.
- V=0 leaves all inferred/design parameters unchanged and P=0.
- Double Ctest at unchanged measured frequencies: Cp/Cs/P double, Lp/Rs halve. Reconstruct both frequency identities independently.
- f1 approaching f0 from below is ill-conditioned but mathematically admissible if the arithmetic remains representable. Test that Cp remains positive and the actual small frequency difference is retained.
- Reject f1>=f0, f1<=0, Ctest<=0, fs<=0 or fs>=f1, ζ outside [0.5,1], k outside [3,4], and negative V. Reject nonfinite arithmetic rather than fabricate a zero component.

### Mandatory diagram

Draw a switch node with parasitic Cp to power return and a separate shunt branch **Rs in series with Cs** to the same return. Show Lp upstream in the ringing loop, not accidentally in series with the snubber alone. Bind component labels to `parasitic_inductance`, `parasitic_capacitance`, `snubber_resistance`, and `snubber_capacitance`. Place a small left inset showing the temporary Ctest connected across Cp during identification, clearly marked “test only; remove before fitting Rs–Cs.” Bind `test_capacitance`. Add two static ringing sketches labeled `original_frequency` and `loaded_frequency`, with the latter visibly slower and captioned illustrative. Bind voltage swing and cycle-loss labels to `voltage_swing` and `snubber_power`. Caption: “Two-frequency identification of one LC mode; temporary test capacitor is replaced by a series-RC shunt snubber. Component values and loss are starting estimates.” No extra count is assigned to identification versus sizing.

## 6. `emc.short_cable_radiation` — intermediate

### Purpose and novelty

Estimate broadside far-field electric interference from an electrically short cable carrying a known uniform in-phase common-mode current. Closest baseline: `signals.line_propagation` predicts propagation delay/wavelength inside an interconnect, not radiated fields. RF explicitly reserves intentional antenna/link calculations and leaves this unintended-emission model to EMC. No cable impedance, skin-depth or antenna gain/radiation-resistance duplicate is added.

### Physical model and limits

MIT equation (10.2.8) gives Eθ=jη0 k I ell exp(−jkr) sinθ/(4πr). At broadside θ=π/2, use η0=μ0 c and k=2πf/c:

- electrical_length = ell f/c
- far_field_parameter = kr=2πfr/c
- electric_field_rms = μ0 f Icm,rms ell/(2r), μ0=4π×1e-7 H/m

Icm is the **sum** of simultaneous conductor currents referenced in the same physical direction. Balanced outgoing/return currents cancel in this source moment; it is not the current in one conductor. The uniform current-length product is an input physical assumption. For a nonuniform cable, one must first establish the appropriate integrated current moment; a current clamp at one point alone does not prove it.

The electrically short condition ell/wavelength<=0.1 and kr>=10 are explicit screening cutoffs for the asymptotic model, not standards requirements. At kr=10 the omitted broadside near-zone terms alter ideal Hertzian magnitude by about 0.5%, but real return conductors/ground reflections may produce much larger error. The result is free space with no ground-image multiplier, not TI's different 10 m chamber geometry. No near-field prediction, arbitrary cable resonance or finite antenna current distribution is implied. The stated conventional μ0 has negligible engineering error compared with these assumptions.

### Complete independent example

Inputs: frequency=100000000 Hz; cable_length=0.1 m; common_mode_current_rms=1e-5 A; distance=10 m.

- wavelength=2.99792458 m (derivation intermediate, not an output).
- electrical_length = 0.1/2.99792458 = **0.03335640951981521**.
- far_field_parameter = 2π×10/2.99792458 = **20.958450219516816**.
- electric_field_rms = (4π×1e-7)×1e8×1e-5×0.1/(2×10) = **6.283185307179587e-6 V/m**.

### Independent tester cases

- Nominal: all three outputs above.
- Double distance to 20 m: electrical_length unchanged; kr doubles; E halves to 3.1415926535897935e-6 V/m.
- Icm=0: field exactly zero; geometry outputs unchanged.
- At f=1e8, ell=0.299792458 m gives ell/wavelength=0.1; with r=10 m it is the admitted short-element endpoint.
- At f=1e8 and nominal ell, r=10×299792458/(2π×1e8) m gives kr=10; a smaller r is rejected, not treated as far field.
- Reject nonpositive f/ell/r, negative RMS current, ell/wavelength>0.1, kr<10 and nonfinite values. Do not reject zero common-mode current.

### Mandatory diagram

Draw a straight cable segment vertically at x=170, y=60..190 with an arrow along it for the net common-mode current, labeled `common_mode_current_rms`; a parallel dimension line binds `cable_length`. Draw a broadside observation ray from cable center to a receiver marker at x=555, y=125; label `distance`. Draw the electric-field arrow at the observer parallel to the cable and perpendicular to the ray, binding `electric_field_rms`. Bind `frequency`, `electrical_length` and `far_field_parameter` in a separate clear legend. Do not add an earth/ground image, which would change the model. Caption: “Uniform in-phase common-mode current element in free space; broadside far-field estimate. Cable return paths and ground reflections are not included.”

## Numerical and implementation handoff

All example values were independently derived using ordinary Python math; coupled-filter gain used complex matrix inversion. No Rust engine, build, test, linter, formatter or desktop checks were executed. The schema uses only existing supported scalar expressions, including `hypot`, and input-only constraints. No engine extension is requested.

Parent/tester should exercise every complete example, symmetry/scaling/zero boundaries and invalid cases above. Use scale-aware relative tolerances for nonzero outputs, exact algebraic zero where attainable, and a justified absolute tolerance only for trigonometric nulls. Input-only finite bounds do not imply every combination remains representable; nonfinite/zero-underflow critical intermediates must report arithmetic errors. No arbitrary practical minimums or maximums should be added to conceal numerical weaknesses.

Six mandatory diagrams must use the existing primitives and exact catalog quantity keys. All diagram geometry is explanatory, not a computed waveform/field plot. Bind only actual quantities, leave 16 px margins, use at least 14 px text, keep components electrically connected through their pins, and separate annotations from geometry. The parent owns registry changes, native image captures, verification and the final novelty gate.

Residual physical risks: slot screening ignores enclosure behavior; the filter omits receiver nonlinearity; harmonic/dither estimates omit receiver weighting; snubber values require bench tuning; cable radiation requires a justified current distribution. These are explicit model boundaries, not unresolved missing implementations. No compliance inference is permitted.
