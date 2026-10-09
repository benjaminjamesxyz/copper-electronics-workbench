# Motors — electromechanical drive features

The Rust catalog owns numerics; Flutter supplies static explanatory diagrams. Models, complete independent examples, sources and limits are promoted from [the research handoff](../../research/expansion/motors.md). No runtime, build, test, lint, formatter or native-image check was run by this developer; Main owns those gates. Hardware ratings and calibration remain separate requirements.

## Source provenance

1. **University lecture, Auburn:** [Modeling Electrical-Mechanical Systems (DC Motors)](https://eng.auburn.edu/~dmbevly/mech3140/Modeling_DC_Motors.pdf), slides 12–16. Read the coupled equations `L dI/dt + R I = Vs - Kb omega`, `J domega/dt + b omega = KI I`, and the stated SI equality `KI=Kb`. Slides 7–11 explain back EMF, inertia, viscous drag and neglected friction/compliance.
2. **University lecture, Ottawa:** [ELG4139 DC Drives](https://www.site.uottawa.ca/~rhabash/ELG4139DCDrives.pdf), torque equation, equivalent circuit and torque-speed slides 4–6. Read `Te-TL=J domega/dt`, steady armature voltage and torque-speed relationship. These add the external shaft load to Auburn's motor-only equation.
3. **Motor manufacturer application note:** [FAULHABER AN015, Microstepping](https://www.faulhaber.com/fileadmin/Import/Media/AN015_EN.pdf), pages 1–3, equations 1–2 and “Accuracy vs. resolution.” Read incremental sine torque, the 16-microstep 9.80% value, and warnings about harmonics, friction and detent torque. The note describes typical detent torque of 5–20% of holding torque; that is a warning, not a universal input bound.
4. **Motor manufacturer technical guide:** [Oriental Motor, How to Address Increased Loads in Both Size and Weight](https://www.orientalmotor.com/stepper-motors/technology/pdf/Increase-Load-Weight.pdf), page 1. Read `Jmotor_referred=Jload/GR^2`, `Tgear= Tmotor GR efficiency` and mechanical strength limits. PDF extraction was incomplete on page 5; no claim relies on that page.
5. **Motor manufacturer sizing guide:** [Oriental Motor, Motor Sizing Calculations](https://www.orientalmotor.com/technology/motor-sizing-calculations.html), “Torque” and “Acceleration Torque.” Read the load-plus-acceleration decomposition and inspected the linked [common acceleration formula image](https://www.orientalmotor.com/images/technology/acceleration-torque-common-formula.jpg). Derivations here use coherent SI Newton–Euler equations rather than the guide's mixed-unit constants. The site's loose wording that inertia is zero when stationary is not adopted: mass moment of inertia does not vanish at rest.
6. **University textbook:** [University of Illinois, Krein, section 19.7.2](https://krein-textbook.ece.illinois.edu/ch19doc.html). Read the back-EMF discussion and the distinction between dissipative dynamic braking and energy-returning regenerative braking. The exponential resistor-brake solution below is independently derived from the coupled Auburn equations; the textbook does not supply that exact closed form.
7. **Engineering-software manufacturer's primary model documentation:** [MathWorks PMSM (DQ0)](https://www.mathworks.com/help/simscape-electrical/ref/pmsmdq0.html), “Equations” and original non-orthogonal Park transform. Read the explicit `2/3` transformation, cross-axis voltage terms, pole-pair conversion and saliency torque. This is primary documentation of the exact mathematical convention used here, not a claim that a particular motor has these parameters.
8. **University course notes:** [MIT 6.685 Electric Machines, Kirtley Class Notes 10](https://ocw.mit.edu/courses/6-685-electric-machines-fall-2013/d834469bd6c3f56c6bec451d76176156_MIT6_685F13_chapter10.pdf), sections 2–2.2, pages 1–5. Read the stator-referred equivalent circuit, RMS rotor-current torque law, Thevenin reduction and peak-torque matching condition. Rotor power splitting below follows the equivalent resistance and conservation of energy.

Searches covered university DC drive equations, manufacturer stepper microstepping, manufacturer gear inertia/sizing, passive braking, PMSM dq conventions and university induction-machine torque/slip. Michigan CTMS returned HTTP 403, a legacy TI conference PDF returned HTTP 410, and a Rice braking laboratory PDF fetch aborted. None is used as evidence or listed in the JSON references. Accessible alternative sources above cover each formula. No motor hardware, datasheet-specific rating or transient simulation was measured.


Source reads and independent derivations above describe the research evidence, not fresh developer measurements.

## 1. `motors.pm_dc_load_equilibrium`

### Purpose and distinct model

Determine the coupled steady operating point, not merely current through a resistor. A load increase changes armature current, winding drop, back EMF and mechanical speed together. Closest baseline is `dc.resistive_source_matching`; a motor's generated EMF is speed-dependent and mechanical drag is not a resistor load. Unlike the geared-ramp item, this model solves an unknown steady speed from imposed voltage and drag; it contains no acceleration or transmission.

### Equations, signs and validity

Sources: Auburn slides 12–16 and Ottawa slides 4–6. In coherent SI, let `K=motor_constant` have numerically equal torque and back-EMF constants. Forward speed and current are positive; `load_torque` and `viscous_drag*speed` oppose rotation.

At equilibrium:

`voltage = armature_resistance*current + K*speed`

`K*current = load_torque + viscous_drag*speed`

Eliminate current:

`speed = (K*voltage-armature_resistance*load_torque)/(K^2+armature_resistance*viscous_drag)`.

Use `current=(load_torque+viscous_drag*speed)/K` rather than subtracting two nearly equal voltages at no load. Then `back_emf=K*speed`, `shaft_power=load_torque*speed`, `copper_loss=R*current^2`, `friction_loss=b*speed^2`. Power identity: `voltage*current = shaft_power+copper_loss+friction_loss`.

Require `R>0`, `K>0`, `b>=0`, `V>=0`, `Tload>=0`, and `K V>=R Tload`. Equality is a mathematical stall equilibrium, not a safe continuous operating condition. Constant flux/resistance and viscous drag exclude brush drops, Coulomb friction, cogging, saturation, thermal feedback and iron loss. Inductance disappears only because this is steady state.

### Complete independent example

Inputs: `voltage=24 V`, `armature_resistance=2 ohm`, `motor_constant=0.1 N m/A`, `viscous_drag=0.001 N m s/rad`, `load_torque=0.2 N m`.

- Numerator `0.1*24-2*0.2=2`; denominator `0.1^2+2*0.001=0.012`.
- `speed=2/0.012=166.66666666666667 rad/s`.
- `current=(0.2+0.001*166.66666666666667)/0.1=3.6666666666666667 A`.
- `back_emf=0.1*166.66666666666667=16.666666666666668 V`.
- `shaft_power=0.2*166.66666666666667=33.333333333333336 W`.
- `copper_loss=2*(11/3)^2=26.88888888888889 W`.
- `friction_loss=0.001*(500/3)^2=27.77777777777778 W`.
- Independent power balance: input `24*(11/3)=88 W`; the three outputs sum to `88 W`.

### Tester cases

- Nominal: all six outputs above and both constitutive equations.
- Ideal unloaded limit: defaults except `load_torque=0, viscous_drag=0` gives `speed=240`, `current=0`, `back_emf=24`, and all three power outputs zero. Do not call this real motor no-load current.
- Stall boundary: defaults except `load_torque=1.2` gives `speed=0`, `current=12`, `back_emf=0`, `shaft_power=0`, `copper_loss=288`, `friction_loss=0`.
- Unenergized limit: `voltage=0, load_torque=0` gives all outputs zero, including with positive drag.
- Reject load above `1.2 N m` at the nominal supply, zero/nonpositive `K` or `R`, and negative supply/load/drag.

### Mandatory diagram

**Topology:** left electrical loop and right mechanical torque balance. Draw a vertical voltage source at approximately `(65,140)`, armature resistor on the upper lead near `(185,80)`, and a back-EMF voltage source near `(305,140)`. Return their lower pins along `y=210`. Both source polarities have positive top terminals. Positive motor current goes along the upper lead from the supply toward the motor; it enters the back-EMF source's positive terminal. Break wires at source/resistor bodies.

Use a dashed, explicitly non-electrical coupling arrow from the motor EMF region toward a rotor circle centered near `(450,140)`. A shaft leads to a load block near `(610,140)`. Above the shaft show the positive speed direction; below, opposite arrows identify load and drag. Keep electrical wiring separate from mechanical arrows.

**Quantity labels:** `voltage` as `V`, `armature_resistance` as `Ra`, `current` as `I`, `back_emf` as `E`, `speed` as `omega`, `load_torque` as `Tload`. Plain text near coupling: `E = K omega; T = K I`. Bind a short `K` label to `motor_constant` if space permits. Plain drag text is `b omega`; a separate `b` label may bind `viscous_drag`, but never bind `b omega` to `viscous_drag`.

**Caption:** “At steady speed, back EMF and winding drop sum to the applied voltage. Electromagnetic torque balances load and viscous drag. Positive current enters the motor; no acceleration or brush drop is modeled.”
