import 'primitives.dart';

const cablesDiagrams = <String, CalculatorDiagram>{
  'cables.twin_lead_geometry': CalculatorDiagram(
    title: 'Balanced round-wire pair in homogeneous dielectric',
    caption: 'D is center spacing; L′ is loop inductance and C′ is between wires. '
        'Dot/cross show opposite longitudinal currents. Field sketch is not to scale; no shield or plane.',
    marks: [
      Part(CircuitPart.circle, 120, 138, width: 72, height: 72),
      Part(CircuitPart.circle, 350, 138, width: 72, height: 72),
      Label(97, 117, '+V/2'),
      Label(327, 117, '−V/2'),
      Dot(120, 153),
      Wire([344, 147, 356, 159]),
      Wire([344, 159, 356, 147]),
      Wire([120, 60, 350, 60], arrow: true, ink: DiagramInk.input),
      Wire([120, 52, 120, 93], dashed: true),
      Wire([350, 52, 350, 93], dashed: true),
      Label(202, 31, 'D', quantity: 'center_spacing'),
      Wire([84, 199, 156, 199], arrow: true, ink: DiagramInk.input),
      Wire([84, 183, 84, 205]),
      Wire([156, 183, 156, 205]),
      Label(104, 212, 'd', quantity: 'wire_diameter'),
      Wire([146, 112, 175, 88, 210, 77, 260, 77, 297, 88, 324, 112], arrow: true, ink: DiagramInk.output),
      Wire([146, 165, 175, 188, 212, 199, 260, 199, 297, 187, 324, 165], arrow: true, ink: DiagramInk.output),
      Label(213, 136, 'E field', ink: DiagramInk.output),
      Label(435, 57, 'Homogeneous εr', quantity: 'relative_permittivity'),
      Label(435, 99, 'Loop L′ [H/m]', quantity: 'inductance_per_length'),
      Label(435, 141, 'Pair C′ [F/m]', quantity: 'capacitance_per_length'),
      Label(435, 183, 'Zdiff [Ω]', quantity: 'differential_impedance'),
      Label(212, 239, 'Cross-section; currents normal to page', ink: DiagramInk.muted),
    ],
  ),
};
