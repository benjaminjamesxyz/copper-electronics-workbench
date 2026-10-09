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

