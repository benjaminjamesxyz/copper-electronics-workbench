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

## 2. `motors.stepper_static_microstep`

### Purpose and distinct model

Separate controller resolution from mechanical accuracy and restoring torque. The baseline clock divider only quantizes electrical timing; it cannot predict rotor lag or torque. This item has a spatial sinusoidal torque law, distinct from all DC/PMSM speed or energy models. It does not calculate a pulse generator or dynamic pull-out speed.

### Equations, signs and validity

Source: FAULHABER AN015, equations 1–2 and the accompanying limitations. Let `S=full_steps_per_turn`, `m=microsteps_per_step`, and `H=holding_torque`. Four full steps span one electrical cycle, so electrical rotor lag is `(S/4)*delta_mechanical`. The ideal torque law is:

`T=H*sin((S/4)*delta_mechanical)`.

Commanded microstep angle is `2*pi/(S*m)`. With the rotor initially aligned and held fixed, one new microstep gives electrical displacement `pi/(2*m)`, hence `incremental_torque=H*sin(pi/(2*m))`.

At a static load `Tload`, use the stable branch:

`load_lag=(4/S)*asin(Tload/H)`.

Differentiate torque with respect to lag:

`angular_stiffness=(S/4)*H*cos(asin(Tload/H))=(S/4)*sqrt(H-Tload)*sqrt(H+Tload)`.

The split roots preserve the vanishing stiffness without first subtracting squared nearly equal torques. Lag is positive behind the commanded current vector. Actual electromagnetic torque opposes a further positive lag; stiffness is its positive restoring magnitude.

Require integer `m>=1`, positive `H`, `0<=Tload<H`, and `S` a positive multiple of four. The last guard is the integer electrical-cycle count for this ideal two-phase model, not an arbitrary bound. Holding torque must correspond to the same current-vector magnitude as microstepping. Real full-step nameplate ratings may use different simultaneous phase-current normalization. Detent torque, current distortion, winding mismatch, friction, resonance and mechanical compliance are omitted. Exact holding torque has zero stiffness and is not admitted as stable operation.

### Complete independent example

Inputs: `full_steps_per_turn=200`, `microsteps_per_step=16`, `holding_torque=0.5 N m`, `load_torque=0.1 N m`.

- Full-step mechanical angle `2*pi/200=0.031415926535897934 rad`.
- `microstep_angle=2*pi/(200*16)=0.001963495408493621 rad`.
- `incremental_torque=0.5*sin(pi/32)=0.0490085701647803 N m`, or `9.80171403295606%` of holding torque.
- Electrical lag `asin(0.1/0.5)=0.2013579207903308 rad`.
- `load_lag=0.02*0.2013579207903308=0.004027158415806616 rad`.
- `angular_stiffness=50*sqrt(0.4)*sqrt(0.6)=24.49489742783178 N m/rad`.

The load lag exceeds one microstep angle; that is a model result, not a rounding fault or evidence of accurate one-microstep physical motion.

### Tester cases

- Nominal: all four outputs above; substitution of `load_lag` into the torque law returns `0.1 N m`.
- Full-step unloaded limit: defaults except `microsteps_per_step=1, load_torque=0` gives angles `0.031415926535897934` and `0`, torque `0.5`, stiffness `25`.
- Increasing `m` reduces `incremental_torque` and command angle but leaves static lag and stiffness unchanged for the same load/current vector.
- As `load_torque` approaches `0.5` from below, `load_lag` approaches `pi/100=0.031415926535897934 rad` and stiffness approaches zero. Do not round an admitted subcritical value up to holding torque.
- Reject `microsteps_per_step=1.5`, `full_steps_per_turn=202`, zero holding torque, negative load and load equal to or exceeding holding torque.

### Mandatory diagram

**Geometry:** a rotor/stator orientation sketch on the left, not an electrical wiring diagram. Use a large circle centered near `(170,135)` and radial direction arrows showing the commanded current-vector direction and the rotor magnetic axis lagging behind. Give the lag arrow a plain `delta` marker and bind `load_lag` to a separate `load lag` label. Make clear that the drawn separation is illustrative, not the computed angle.

On the right, draw horizontal mechanical-lag and vertical restoring-torque axes within `(380,65)` to `(660,215)`. Use a static polyline following the first increasing quarter of a sine, with zero torque at zero lag and a flat top at the holding limit. Mark a conceptual operating point with a dot and a short local tangent; this is not positioned from live input. Below the left sketch show two nearby field-direction rays separated by a conceptual microstep.

**Quantity labels:** `holding_torque` as `Th`, `load_torque` as `Tload`, `load_lag` as `load lag`, `angular_stiffness` as `local stiffness`, `microstep_angle` as `command step`, `incremental_torque` as `Tinc`. `full_steps_per_turn` and `microsteps_per_step` may be left to the legend to avoid crowding. Plain axis endpoint relation: `full-step lag = 2 pi/S`; do not bind this relation to `microstep_angle`.

**Caption:** “Illustrative sinusoidal torque versus rotor lag. Smaller current-vector steps improve command resolution, not guaranteed positioning accuracy. Tinc starts from unloaded alignment; detent torque and friction are omitted.”

## 3. `motors.geared_axis_acceleration`

### Purpose and distinct model

Translate a required mechanical ramp into motor drive current and voltage. The distinguishing laws are two-inertia acceleration, ratio-squared kinetic-energy reflection and lossy transmission torque. This is not an inverse of the steady DC load model: both shaft speeds are prescribed, torque accelerates inertia, and gearbox efficiency acts only across the transmission.

### Equations, signs and validity

Sources: Oriental Motor gearing page 1 and acceleration guide; Auburn equations for current/voltage. Define `n=gear_ratio=omega_motor/omega_load`, positive for the chosen corresponding shaft directions. Gear reversal signs are absorbed in that orientation definition. Let `alpha_load=(final_speed-initial_speed)/ramp_time`.

Kinetic energy of the load is `Jl*(omega_motor/n)^2/2`, so `reflected_inertia=Jl/n^2`. Input rotor acceleration is `motor_acceleration=n*alpha_load`.

Output gear torque must supply `Jl*alpha_load+Tload`. The forward-efficiency relation gives required gear-input torque `(Jl*alpha_load+Tload)/(eta*n)`. Add the motor/input-side inertia torque before the gear:

`motor_torque=Jm*n*alpha_load+(Jl*alpha_load+Tload)/(eta*n)`.

Then `armature_current=motor_torque/K`, `motor_final_speed=n*final_speed`, and `required_final_voltage=K*motor_final_speed+Ra*armature_current`. This is the end-of-ramp voltage immediately before acceleration stops, not after the current demand drops.

Require nonnegative inertias with positive sum; `n>0`; `0<eta<=1`; nonnegative opposing load and initial/final speed; `final_speed>=initial_speed`; positive ramp time, `K` and `Ra`. An ideal equal-speed case is accepted. Efficiency does not alter physical `reflected_inertia`. The motor inertia includes input-referred gear inertia. No backdrive efficiency, compliance, backlash, friction beyond `load_torque`, torque-ripple or transient current-edge voltage is predicted.

### Complete independent example

Inputs: `motor_inertia=0.0002 kg m^2`, `load_inertia=0.02 kg m^2`, `gear_ratio=10`, `gear_efficiency=0.9`, `load_torque=0.4 N m`, `initial_speed=0 rad/s`, `final_speed=20 rad/s`, `ramp_time=0.5 s`, `motor_constant=0.08 N m/A`, `armature_resistance=1 ohm`.

- Load acceleration `(20-0)/0.5=40 rad/s^2`.
- `reflected_inertia=0.02/10^2=0.0002 kg m^2`.
- `motor_acceleration=10*40=400 rad/s^2`.
- Motor-inertia torque `0.0002*400=0.08 N m`.
- Gear-output demand `0.02*40+0.4=1.2 N m`; input demand `1.2/(0.9*10)=0.13333333333333333 N m`.
- `motor_torque=0.08+0.13333333333333333=0.21333333333333335 N m`.
- `armature_current=0.21333333333333335/0.08=2.6666666666666665 A`.
- `motor_final_speed=10*20=200 rad/s`.
- `required_final_voltage=0.08*200+1*2.6666666666666665=18.666666666666668 V`.

### Tester cases

- Nominal: all six outputs above. Verify torque on both sides of the gearbox independently.
- Lossless gear: change only `gear_efficiency=1`; outputs are `reflected_inertia=0.0002`, `motor_acceleration=400`, `motor_torque=0.2`, `armature_current=2.5`, `motor_final_speed=200`, `required_final_voltage=18.5`.
- Constant speed: set `initial_speed=final_speed=20`; acceleration becomes zero, torque is `0.4/9=0.044444444444444446 N m`, current `0.5555555555555556 A`, end voltage `16.555555555555557 V`; inertia output remains `0.0002`.
- `gear_ratio=1, gear_efficiency=1` reduces torque to `(Jm+Jl)*alpha_load+Tload`, not to just load torque.
- Reject a decreasing speed ramp, zero duration, zero ratio, efficiency outside `(0,1]`, negative inertias or two zero inertias. Do not reject a zero load torque or one zero inertia.

### Mandatory diagram

**Energy/mechanical chain:** three blocks across the center: PM motor/input inertia at `(150,130)`, gearbox at `(350,130)`, output inertia/load at `(560,130)`. Shafts/arrows connect block edges, with no electrical-connection implication. A current arrow enters the motor from above; its voltage label sits beside it. The gearbox is labeled with `n` and `eta`, not as a torque multiplier equal to `n` alone. Mark positive acceleration and speed arrows above both shafts. A smaller lower-left annotation shows `Jl/n^2` reflected to the motor shaft with a dashed arrow, not a second physical inertia component.

**Quantity labels:** `armature_current` as `I`, `required_final_voltage` as `V end`, `motor_inertia` as `Jm`, `gear_ratio` as `n`, `gear_efficiency` as `eta`, `load_inertia` as `Jl`, `load_torque` as `Tload`, `motor_torque` as `Tm`, `reflected_inertia` as `J reflected`. Use a plain relation `omega_m = n omega_l`; bind `motor_final_speed` and `final_speed` to their own short speed labels if space permits.

**Caption:** “Forward motoring ramp through a fixed gear. Reflected inertia scales as 1/n²; output acceleration and load torque cross the efficiency boundary. Voltage is evaluated before the ramp ends and excludes current-edge inductive voltage.”
