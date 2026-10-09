import 'primitives.dart';

const rfDiagrams = <String, CalculatorDiagram>{
  'rf.matched_t_attenuator': CalculatorDiagram(
    title: 'Equal-reference matched T pad',
    caption: 'Both series resistors equal Rs. Power transmission applies with the '
        'output terminated in Z0; ideal lumped resistors only. Common return is '
        'the lower rail. Geometry is schematic, not to scale.',
    marks: [
      Label(55, 55, 'Input Z0', quantity: 'reference_impedance', ink: DiagramInk.input),
      Label(288, 20, 'Loss A', quantity: 'attenuation_db', ink: DiagramInk.input),
      Wire([175, 47, 495, 47], arrow: true, ink: DiagramInk.input),
      Label(202, 73, 'Rs', quantity: 'series_resistance', ink: DiagramInk.output),
      Label(427, 73, 'Rs', quantity: 'series_resistance', ink: DiagramInk.output),
      Wire([55, 105, 185, 105]),
      Part(CircuitPart.resistor, 215, 105),
      Wire([245, 105, 410, 105]),
      Part(CircuitPart.resistor, 440, 105),
      Wire([470, 105, 590, 105]),
      Dot(55, 105),
      Dot(325, 105),
      Dot(590, 105),
      Wire([325, 105, 325, 130]),
      Part(CircuitPart.resistor, 325, 160, turns: 1),
      Wire([325, 190, 325, 220]),
      Label(349, 151, 'Rp', quantity: 'shunt_resistance', ink: DiagramInk.output),
      Wire([590, 105, 590, 130]),
      Part(CircuitPart.resistor, 590, 160, turns: 1),
      Wire([590, 190, 590, 220]),
      Label(612, 151, 'Z0', quantity: 'reference_impedance', ink: DiagramInk.input),
      Label(565, 73, 'Output'),
      Wire([55, 220, 590, 220]),
      Dot(55, 220),
      Dot(325, 220),
      Dot(590, 220),
      Label(55, 240, 'Common return', ink: DiagramInk.muted),
      Label(463, 240, 'Pout/Pin', quantity: 'power_transmission', ink: DiagramInk.output),
    ],
  ),
};
