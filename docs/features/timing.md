# Digital timing interfaces and EMC — executable calculators

These eight offline numerical models execute in the shared Rust engine; Flutter consumes catalog metadata and does not duplicate calculations. Units are SI unless stated as `%`, dimensionless `1`, or baud (symbols/s). Levels are two basic, three intermediate and three expert. Negative pullup interval width, UART margin and setup slack are intentional failed-design results, not input errors. These are engineering estimates, not simulation or standards-compliance certificates.

The derivations and independently computed nominal examples are retained from [timing research](../../research/timing.md); the executable definitions are [backend/catalog/timing.json](../../backend/catalog/timing.json). Each section lists every input and exact expected output with relative tolerance **1e-6**. Example inputs are also the defaults. Input-only constraints execute before sequential outputs; finite inputs, required keys, bounds and nonfinite intermediate/final arithmetic are enforced by the shared engine.

Sources are primary manufacturer/university material. Research directly read the cited TI, MIT and UMBC sources. Microchip CTC/UART references were available as indexed manufacturer content through web search; direct Microchip HTML retrieval returned HTTP 503 during research. No new direct-source verification or engine/build/test execution is claimed by this implementation.

Boundary distinctions: Schmitt thresholds and intrinsic gate delay distinguish digital RC timing from general analog charging. Timer quantization differs from asynchronous UART drift. Setup slack is not routed trace skew. Loaded-victim capacitive crosstalk differs from dielectric-overlap displacement current, and the specified-slope inductive spike does not predict an RL decay or gate-drive transition.

Numerical limits: physical positive denominators do not guarantee representable floating-point products at arbitrary finite magnitudes; the engine must explicitly reject nonfinite or invalid intermediates. Near-zero differences such as timing slack or threshold windows must be interpreted against hardware uncertainty. At extremely small threshold/supply ratios the Schmitt logarithmic subtraction can lose precision; the illustrative examples are well conditioned. Crosstalk uses native `expm1` to preserve fast-edge physics rather than adding an artificial minimum edge/time-constant ratio. A ratio that actually underflows to zero remains an explicit arithmetic-domain failure, not a physical-domain restriction.

## 3. 16-bit toggle timer clock divider — basic

**ID:** `timing.timer_clock_divider`

**Purpose:** Choose a realizable compare-register value for a supplied hardware prescaler and report actual output frequency and quantization error.

**Derivation:** A timer counts inclusively from 0 through compare value `K`, taking `K+1` prescaled clock ticks. Toggle output changes state once per interval, requiring two intervals for a complete square-wave cycle:

`f_actual = f_clock/[2 P(K+1)]`.

The ideal count is `f_clock/(2 P f_target)`. Choose nearest count, half-integer ties upward:

`count = floor(f_clock/(2 P f_target)+0.5)`; `compare_value = count-1`.

Then `error_percent = 100(f_actual/f_target - 1)` and `output_period = 1/f_actual`.

**Dimensions/domain:** Source/output frequency in Hz, count/compare/prescaler dimensionless, period in s, error in %. Positive source and target frequencies; target cannot exceed `f_clock/(2P)`, so rounded count cannot be zero. Rounded count must be at most 65536 for a 16-bit compare register. The polynomial equality `(P-1)(P-8)(P-64)(P-256)(P-1024)=0` restricts P to the five common AVR-style prescalers without conditional expressions. Verify the chosen timer's actual prescaler availability and toggle mode. This is nearest-period rounding, **not** a claim of mathematically minimum frequency error. Prescaler is supplied, not searched. Clock tolerance and safe register updating are separate design concerns.

**Independent example:** 16 MHz source, P = 8, target 3 kHz:

- Ideal count `= 16e6/(16 × 3000) = 333.333333333`.
- Rounded count `= 333`, compare `= 332`.
- Period `= 2 × 8 × 333 / 16e6 = 333 us`.
- Actual frequency `= 1/333e-6 = 3003.003003003 Hz`.
- Error `= (3003.003003003 - 3000)/3000 × 100 = +0.100100100100%`.

**Primary sources:** [Microchip AVR CTC mode documentation](https://onlinedocs.microchip.com/oxy/GUID-80B1922D-872B-40C8-A8A5-0CBE009FD908-en-US-3/GUID-6CCB4482-FFD4-4AD9-9D5C-DFC302038433.html); [Microchip ATmega640/1280/1281/2560/2561 datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/ATmega640-1280-1281-2560-2561-Datasheet-DS40002211A.pdf), timer CTC frequency relation. The register-rounding policy is this calculator's explicit design choice.


### Catalog inputs and complete worked outputs

| Input key | Label | Unit | Example/default | Admissible range |
|---|---|---|---|---|
| `clock_hz` | Timer source clock | Hz | `16000000` | > 0 |
| `prescaler` | Hardware prescaler | 1 | `8` | >= 1; <= 1024 |
| `target_hz` | Target square-wave frequency | Hz | `3000` | > 0 |

Cross-input/model constraints (evaluated before outputs):

- `(prescaler-1)*(prescaler-8)*(prescaler-64)*(prescaler-256)*(prescaler-1024) == 0` — Prescaler must be one of 1, 8, 64, 256 or 1024.
- `target_hz <= clock_hz/(2*prescaler)` — Target exceeds the maximum toggle-output frequency.
- `floor(clock_hz/(2*prescaler*target_hz)+0.5) <= 65536` — Rounded count exceeds the 16-bit compare-register range.

| Sequential output key | Label | Unit | Exact independent expected value | Executable expression |
|---|---|---|---|---|
| `count` | Timer clocks per toggle | 1 | `333` | `floor(clock_hz/(2*prescaler*target_hz)+0.5)` |
| `compare_value` | Compare register value | 1 | `332` | `count-1` |
| `actual_hz` | Actual output frequency | Hz | `3003.003003003003` | `clock_hz/(2*prescaler*count)` |
| `error_percent` | Signed frequency error | % | `0.1001001001001001` | `100*(actual_hz/target_hz-1)` |
| `output_period` | Actual output period | s | `0.000333` | `1/actual_hz` |

