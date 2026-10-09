import 'primitives.dart';

const sensorsDiagrams = <String, CalculatorDiagram>{
  'sensors.ntc_steinhart_hart': CalculatorDiagram(
    title: 'Calibrated NTC resistance and inverse sensitivity',
    caption: 'Calibrated Steinhart–Hart thermistor law; curve and tangent are illustrative, not computed. Sense excitation must not materially heat the bead. Reported dT/dR is the inverse of the plotted R-versus-T slope.',
    marks: [
      Wire([70, 130, 120, 130]),
      Part(CircuitPart.resistor, 150, 130),
      Wire([180, 130, 230, 130]),
      Wire([128, 155, 174, 105, 188, 105]),
      Label(70, 172, 'R measured', quantity: 'resistance'),
      Wire([150, 62, 150, 101], arrow: true, ink: DiagramInk.input),
      Label(175, 65, 'T (K)', quantity: 'temperature_kelvin'),
      Label(45, 215, '1/T = A + B ln(R) + C ln³(R)'),
      Label(45, 245, 'A', quantity: 'coefficient_a'),
      Label(130, 245, 'B', quantity: 'coefficient_b'),
      Label(215, 245, 'C', quantity: 'coefficient_c'),
      Wire([375, 200, 650, 200], arrow: true),
      Wire([375, 200, 375, 35], arrow: true),
      Label(350, 25, 'R'),
      Label(645, 212, 'T'),
      Wire([395, 50, 415, 88, 440, 121, 470, 145, 510, 165, 560, 180, 625, 188], ink: DiagramInk.output),
      Wire([425, 124, 525, 174], dashed: true, ink: DiagramInk.muted),
      Dot(470, 145),
      Label(505, 105, 'Inverse tangent:'),
      Label(505, 127, 'dT/dR', quantity: 'temperature_sensitivity'),
      Label(400, 242, 'Illustrative R(T), not a live plot'),
    ],
  ),
};
