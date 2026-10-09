import 'primitives.dart';

const passivesDiagrams = <String, CalculatorDiagram>{
  'passives.mlcc_logarithmic_aging': CalculatorDiagram(
    title: 'Ferroelectric relaxation after deaging',
    caption: 'Conceptual semilog aging law, not a live-scaled plot. Heating above '
        'the Curie point resets the clock. Fixed temperature and DC bias; '
        'aging is relative to the one-hour value, not compounded.',
    marks: [
      Wire([70, 50, 70, 205, 520, 205], arrow: true),
      Label(35, 42, 'C'),
      Wire([85, 85, 485, 155], ink: DiagramInk.output),
      Dot(85, 85),
      Dot(485, 155, ink: DiagramInk.output),
      Wire([85, 85, 85, 205], dashed: true, ink: DiagramInk.muted),
      Wire([485, 155, 485, 205], dashed: true, ink: DiagramInk.muted),
      Label(95, 57, 'C₁h', quantity: 'capacitance_one_hour'),
      Label(390, 125, 'C(t)', quantity: 'aged_capacitance', ink: DiagramInk.output),
      Label(78, 215, '1 h'),
      Label(475, 215, 't', quantity: 'age'),
      Label(185, 238, 'Time since deaging (log scale)'),
      Label(185, 160, 'A: % of C₁h / decade', quantity: 'aging_percent_decade'),
      Part(CircuitPart.capacitor, 590, 90, width: 60, height: 36),
      Wire([545, 90, 560, 90]),
      Wire([620, 90, 640, 90]),
      Label(541, 135, 'Ferroelectric'),
      Label(553, 158, 'dielectric'),
    ],
  ),
};
