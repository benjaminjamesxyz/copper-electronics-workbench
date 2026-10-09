import 'primitives.dart';

const precisionDiagrams = <String, CalculatorDiagram>{
  'precision.bjt_mirror_base_current': CalculatorDiagram(
    title: 'Two-base-current mirror deficit',
    caption: 'Reference current supplies one collector and two bases. The output collector stays at or above the base voltage; matched DC devices and no Early effect are assumed. Static circuit, not a measured operating point.',
    marks: [
      Part(CircuitPart.bjt, 245, 150, width: 60, height: 70),
      Part(CircuitPart.bjt, 445, 150, width: 60, height: 70),
      Wire([245, 45, 245, 95], arrow: true),
      Wire([245, 95, 245, 115]),
      Wire([245, 95, 185, 95, 185, 150, 215, 150]),
      Wire([185, 150, 185, 205, 365, 205, 365, 150, 415, 150]),
      Dot(245, 95), Dot(185, 150),
      Wire([445, 45, 445, 115], arrow: true),
      Wire([245, 185, 245, 235, 445, 235, 445, 185]),
      Dot(345, 235),
      Wire([345, 235, 345, 244]),
      Part(CircuitPart.ground, 345, 254, width: 26, height: 20),
      Wire([373, 150, 405, 150], arrow: true),
      Label(155, 18, 'Iref', quantity: 'reference_current'),
      Label(454, 48, 'Ic', quantity: 'collector_current'),
      Label(454, 82, 'Vc', quantity: 'collector_voltage'),
      Label(375, 124, 'Ib each', quantity: 'base_current_each'),
      Label(258, 145, 'Q1'), Label(458, 145, 'Q2'),
      Label(28, 173, 'Base = VBE', quantity: 'base_emitter_voltage'),
      Label(28, 223, 'Matched beta', quantity: 'beta'),
      Label(510, 154, 'Deficit / Iref', quantity: 'fractional_deficit'),
      Label(510, 198, 'Margin = Vc − VBE', quantity: 'compliance_margin'),
      Wire([492, 95, 502, 95, 502, 150, 492, 150], dashed: true),
    ],
  ),
};
