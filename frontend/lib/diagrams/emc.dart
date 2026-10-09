import 'primitives.dart';

const emcDiagrams = <String, CalculatorDiagram>{
  'emc.thin_slot_shielding': CalculatorDiagram(
    title: 'One isolated thin slot in a conducting shield',
    caption: 'One thin, electrically small slot; worst-orientation field-leakage estimate. '
        'Enclosure resonances and other openings are not modeled. Static geometry, not to scale; '
        'arrows pass through the opening, not through metal.',
    marks: [
      Wire([170, 65, 430, 65, 430, 200, 170, 200, 170, 65]),
      Wire([205, 115, 395, 115, 395, 140, 205, 140, 205, 115]),
      Label(215, 170, 'Conducting shield face'),
      Wire([205, 92, 395, 92], arrow: true, ink: DiagramInk.input),
      Wire([205, 86, 205, 100]), Wire([395, 86, 395, 100]),
      Label(275, 69, 'L', quantity: 'slot_length'),
      Wire([408, 115, 408, 140], arrow: true),
      Label(438, 115, 'w', quantity: 'slot_width'),
      Wire([250, 157, 310, 122], arrow: true, ink: DiagramInk.input),
      Wire([310, 122, 370, 101], arrow: true, ink: DiagramInk.output),
      Label(22, 100, 'Incident field'),
      Label(22, 124, 'f', quantity: 'frequency'),
      Label(22, 151, 'λ', quantity: 'wavelength'),
      Label(510, 43, 'Wall edge inset'),
      Wire([550, 83, 565, 83, 565, 171, 550, 171, 550, 83]),
      Wire([550, 193, 565, 193], arrow: true),
      Wire([550, 183, 550, 200]), Wire([565, 183, 565, 200]),
      Label(585, 181, 't', quantity: 'shield_thickness'),
      Label(170, 224, 'SE (dB)', quantity: 'shielding_db', ink: DiagramInk.output),
      Label(360, 224, 'E transmitted / E incident', quantity: 'field_ratio', ink: DiagramInk.output),
    ],
  ),
};
