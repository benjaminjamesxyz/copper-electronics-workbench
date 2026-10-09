import 'primitives.dart';

const optoDiagrams = <String, CalculatorDiagram>{
  'opto.photodiode_quantum_conversion': CalculatorDiagram(
    title: 'Photon-to-carrier conversion',
    caption: 'Monochromatic light generates reverse photocurrent; external quantum efficiency includes optical and collection losses. Unity gain; dark current and readout circuitry omitted. Geometry is illustrative, not to scale.',
    marks: [
      Wire([55, 115, 230, 115], arrow: true, ink: DiagramInk.input),
      Label(55, 68, 'P', quantity: 'optical_power', ink: DiagramInk.input),
      Label(115, 68, 'λ', quantity: 'wavelength', ink: DiagramInk.input),
      Part(CircuitPart.block, 330, 130, width: 180, height: 110),
      Wire([290, 75, 290, 185]),
      Wire([370, 75, 370, 185]),
      Label(256, 90, 'P'),
      Label(296, 90, 'Depletion'),
      Label(387, 90, 'N'),
      Wire([394, 150, 267, 150], arrow: true, ink: DiagramInk.output),
      Label(245, 199, 'Anode'),
      Label(368, 199, 'Cathode'),
      Label(450, 84, 'η', quantity: 'quantum_efficiency', ink: DiagramInk.input),
      Label(450, 120, 'R', quantity: 'responsivity', ink: DiagramInk.output),
      Label(450, 156, 'Iph', quantity: 'photocurrent', ink: DiagramInk.output),
      Label(170, 241, 'Generated conventional current: N → P (not electron flow)'),
    ],
  ),
};
