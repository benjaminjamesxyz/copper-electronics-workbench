# Research board — reserved ownership

128 planned unique calculations: eight per lane. Researchers must not repeat another lane’s physical problem. The sprint-master will reconcile completed catalogs before development.

| Lane | Domain | Reserved boundary | Findings |
|---|---|---|---|
| dc | DC circuits and resistor networks | Ohm law, resistor power, loaded dividers, series/parallel resistors, source resistance, Wheatstone bridge; exclude temperature/noise | [Research](research/dc.md), [Definitions](research/dc.json) |
| ac | AC impedance and resonance | reactance, RLC impedance/magnitude/phase, series/parallel resonance, Q, bandwidth, power factor; exclude filter synthesis | [Research](research/ac.md), [Definitions](research/ac.json) |
| transients | Time-domain passive transients | RC/RL charge/discharge/time targets, stored energy, damped RLC transient; exclude filters and switch-mode converter sizing | [Research](research/transients.md), [Definitions](research/transients.json) |
| diodes | Diodes LEDs and rectification | LED resistor, diode Shockley current, bridge loss, rectifier ripple, zener sizing, clipping, LED parallel branch design; exclude BJT/MOSFET | [Research](research/diodes.md), [Definitions](research/diodes.json) |
| bjt | Bipolar transistor design | bias, base resistor, small signal gm/rpi, emitter degeneration, saturation switch, collector power, Early output resistance | [Research](research/bjt.md), [Definitions](research/bjt.json) |
| mosfet | MOSFET and gate drive | gate charge/current, conduction/switching loss, gate resistance, Miller time, linear-region current, bootstrap capacitance; exclude thermal temperature calculations | [Research](research/mosfet.md), [Definitions](research/mosfet.json) |
| opamps | Operational amplifier stages | inverting/noninverting/difference/summing gains, integrator/differentiator, slew rate, gain bandwidth; exclude noise and filters | [Research](research/opamps.md), [Definitions](research/opamps.json) |
| filters | Analog filter design | first/second order low/high/band pass, Butterworth cutoff, Sallen Key Q, notch, cascaded response; exclude bare AC resonance | [Research](research/filters.md), [Definitions](research/filters.json) |
| power | Power conversion and regulation | buck/boost duty ripple inductance capacitor, LDO dissipation, efficiency/input current; exclude diode-specific rectifier calculations | [Research](research/power.md), [Definitions](research/power.json) |
| magnetics | Magnetics and transformers | turns ratio, flux density, toroid inductance, air-gap inductance, magnetic energy, skin depth, transformer VA, coupled inductance | [Research](research/magnetics.md), [Definitions](research/magnetics.json) |
| signals | Signal integrity and transmission lines | microstrip/stripline impedance, delay, reflections/termination, coax impedance, rise-time bandwidth, wavelength; exclude PCB current and EMC coupling | [Research](research/signals.md), [Definitions](research/signals.json) |
| pcb | PCB interconnect and manufacturing | trace resistance/drop, IPC-2221 current estimate with caveat, via resistance, copper thickness conversion, differential skew, annular ring, trace Joule heating; exclude RF impedance | [Research](research/pcb.md), [Definitions](research/pcb.json) |
| thermal | Thermal and reliability design | junction temperature, heatsink resistance, thermal path, transient thermal lump, Arrhenius acceleration, derating, resistor temperature coefficient, thermal expansion | [Research](research/thermal.md), [Definitions](research/thermal.json) |
| data | ADC DAC and sampled systems | LSB, quantization error/SNR, ENOB, sampling/Nyquist, alias frequency, DAC output, acquisition settling; exclude analog noise budgets | [Research](research/data.md), [Definitions](research/data.json) |
| noise | Noise measurement and analog budgets | Johnson/shot noise, RMS noise sum, amplifier noise, dB/linear, dynamic range, noise figure/Friis; exclude ADC SNR | [Research](research/noise.md), [Definitions](research/noise.json) |
| timing | Digital timing interfaces and EMC | I2C pullup bounds, RC edge threshold timing, clock divider, UART error, setup slack, capacitive crosstalk, inductive spike, displacement current; exclude analog passive charge general cases | [Research](research/timing.md), [Definitions](research/timing.json) |
